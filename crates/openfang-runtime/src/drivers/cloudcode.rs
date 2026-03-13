//! Google Cloud Code Assist driver (cloudcode-pa.googleapis.com).
//! Free Gemini access via OAuth2 tokens. Wrapper format for v1internal endpoint.

use crate::llm_driver::{CompletionRequest, CompletionResponse, LlmDriver, LlmError, StreamEvent};
use async_trait::async_trait;
use openfang_types::message::{ContentBlock, Role, StopReason, TokenUsage};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tracing::debug;

pub struct CloudCodeDriver {
    project_id: String,
    access_token: Mutex<String>,
    refresh_token: String,
    client_id: String,
    client_secret: String,
    base_url: String,
    client: reqwest::Client,
}

#[derive(Deserialize)]
struct TokenResp { access_token: String, expires_in: Option<u64> }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CCReq { project: String, model: String, request: Inner, user_agent: String, request_id: String }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Inner {
    contents: Vec<GCont>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<SysI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GConf>,
}

#[derive(Serialize)]
struct GCont { role: String, parts: Vec<GP> }
#[derive(Serialize, Deserialize)]
struct GP { #[serde(skip_serializing_if = "Option::is_none")] text: Option<String> }
#[derive(Serialize)]
struct SysI { parts: Vec<GP> }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GConf {
    #[serde(skip_serializing_if = "Option::is_none")] max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")] temperature: Option<f32>,
}

#[derive(Deserialize)]
struct GResp { candidates: Option<Vec<GCand>>, #[serde(rename = "usageMetadata")] usage: Option<GUse> }
#[derive(Deserialize)]
struct GCand { content: Option<GCC> }
#[derive(Deserialize)]
struct GCC { parts: Option<Vec<GP>> }
#[derive(Deserialize)]
struct GUse { #[serde(rename = "promptTokenCount")] input: Option<u64>, #[serde(rename = "candidatesTokenCount")] output: Option<u64> }

const UA: &str = "google-cloud-sdk vscode_cloudshelleditor/0.1";
const GAC: &str = "gl-node/22.17.0";
const META: &str = r#"{"ideType":"IDE_UNSPECIFIED","platform":"PLATFORM_UNSPECIFIED","pluginType":"GEMINI"}"#;
// Client ID from GEMINI_OAUTH_CLIENT_ID env var (default: Gemini CLI public client)
// Client secret from GEMINI_OAUTH_CLIENT_SECRET env var

impl CloudCodeDriver {
    pub fn new(credentials: String, base_url: String) -> Self {
        #[derive(Deserialize)]
        struct C { token: Option<String>, access: Option<String>, refresh: Option<String>, #[serde(rename = "projectId")] pid: Option<String> }
        let (tok, ref_, proj) = serde_json::from_str::<C>(&credentials).map(|c|
            (c.token.or(c.access).unwrap_or_default(), c.refresh.unwrap_or_default(), c.pid.unwrap_or_default())
        ).unwrap_or((credentials, String::new(), String::new()));
        Self { project_id: proj, access_token: Mutex::new(tok), refresh_token: ref_,
               client_id: std::env::var("GEMINI_OAUTH_CLIENT_ID").unwrap_or_else(|_| "see-env".to_string()), client_secret: std::env::var("GEMINI_OAUTH_CLIENT_SECRET").unwrap_or_else(|_| "see-env".to_string()), base_url,
               client: reqwest::Client::builder().user_agent(UA).build().unwrap_or_default() }
    }

    async fn refresh(&self) -> Result<String, LlmError> {
        if self.refresh_token.is_empty() {
            return Err(LlmError::AuthenticationFailed("No refresh token".into()));
        }
        debug!("Refreshing Google OAuth2 token...");
        let r = self.client.post("https://oauth2.googleapis.com/token")
            .form(&[("grant_type","refresh_token"),("refresh_token",&self.refresh_token),("client_id",&self.client_id),("client_secret",&self.client_secret)])
            .send().await.map_err(|e| LlmError::Http(format!("Refresh: {e}")))?;
        let b = r.text().await.map_err(|e| LlmError::Http(e.to_string()))?;
        let tr: TokenResp = serde_json::from_str(&b).map_err(|_| LlmError::AuthenticationFailed(format!("Refresh failed: {b}")))?;
        let n = tr.access_token.clone();
        if let Ok(mut t) = self.access_token.lock() { *t = n.clone(); }
        debug!("Token refreshed, expires in {}s", tr.expires_in.unwrap_or(0));
        Ok(n)
    }
}

#[async_trait]
impl LlmDriver for CloudCodeDriver {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse, LlmError> {
        let mut sys = request.system.clone();
        let mut contents = Vec::new();
        for msg in &request.messages {
            match msg.role {
                Role::System => { if sys.is_none() { sys = Some(msg.content.text_content()); } }
                Role::User => contents.push(GCont { role: "user".into(), parts: vec![GP { text: Some(msg.content.text_content()) }] }),
                Role::Assistant => contents.push(GCont { role: "model".into(), parts: vec![GP { text: Some(msg.content.text_content()) }] }),
                _ => {}
            }
        }
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis();
        let body = CCReq {
            project: self.project_id.clone(), model: request.model.clone(),
            request: Inner {
                contents,
                system_instruction: sys.map(|s| SysI { parts: vec![GP { text: Some(s) }] }),
                generation_config: Some(GConf { max_output_tokens: Some(request.max_tokens), temperature: Some(request.temperature) }),
            },
            user_agent: "pi-coding-agent".into(),
            request_id: format!("pi-{ts}-family"),
        };
        let url = format!("{}/v1internal:generateContent", self.base_url);

        for attempt in 0..2u8 {
            let tok = if attempt == 0 { self.access_token.lock().unwrap().clone() } else { self.refresh().await? };
            let r = self.client.post(&url)
                .header("Authorization", format!("Bearer {tok}"))
                .header("X-Goog-Api-Client", GAC)
                .header("Client-Metadata", META)
                .json(&body).send().await.map_err(|e| LlmError::Http(format!("CloudCode: {e}")))?;
            let st = r.status();
            let txt = r.text().await.map_err(|e| LlmError::Http(e.to_string()))?;
            if st.as_u16() == 401 && attempt == 0 { debug!("401, refreshing..."); continue; }
            if st.as_u16() == 429 { return Err(LlmError::RateLimited { retry_after_ms: 5000 }); }
            if !st.is_success() { return Err(LlmError::Api { status: st.as_u16(), message: txt }); }

            let gr: GResp = serde_json::from_str(&txt).map_err(|e| LlmError::Parse(format!("{e}: {txt}")))?;
            let out = gr.candidates.and_then(|c| c.into_iter().next()).and_then(|c| c.content)
                .and_then(|c| c.parts).and_then(|p| p.into_iter().next()).and_then(|p| p.text).unwrap_or_default();
            let usage = gr.usage.map(|u| TokenUsage { input_tokens: u.input.unwrap_or(0), output_tokens: u.output.unwrap_or(0) })
                .unwrap_or(TokenUsage { input_tokens: 0, output_tokens: 0 });
            return Ok(CompletionResponse { content: vec![ContentBlock::Text { text: out, provider_metadata: None }], stop_reason: StopReason::EndTurn, tool_calls: vec![], usage });
        }
        Err(LlmError::AuthenticationFailed("Token refresh failed".into()))
    }
}

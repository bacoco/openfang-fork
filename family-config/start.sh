#!/bin/bash
# OpenFang Family Startup Script
# Set environment variables before running, or edit the defaults below

# === Required ===
export ZAI_API_KEY="${ZAI_API_KEY:-YOUR_ZAI_API_KEY_HERE}"
export BRAVE_SEARCH_API_KEY="${BRAVE_SEARCH_API_KEY:-YOUR_BRAVE_API_KEY_HERE}"

# === Recommended ===
export BYTEPLUS_API_KEY="${BYTEPLUS_API_KEY:-YOUR_BYTEPLUS_API_KEY_HERE}"

# === Gemini OAuth (optional) ===
# If you have a refresh token from Gemini CLI or OpenClaw:
REFRESH="${GEMINI_REFRESH_TOKEN:-}"
PROJECT="${GEMINI_PROJECT_ID:-}"

if [ -n "$REFRESH" ] && [ -n "$PROJECT" ]; then
  # Auto-refresh the access token
  TOKEN=$(curl -s -X POST "https://oauth2.googleapis.com/token" \
    -d "grant_type=refresh_token&refresh_token=$REFRESH&client_id=${GEMINI_OAUTH_CLIENT_ID}&client_secret=${GEMINI_OAUTH_CLIENT_SECRET}" \
    | node -e "process.stdout.write(JSON.parse(require('fs').readFileSync('/dev/stdin','utf8')).access_token||'')")
  export GEMINI_CLOUDCODE_CREDENTIALS="{\"token\":\"$TOKEN\",\"refresh\":\"$REFRESH\",\"projectId\":\"$PROJECT\"}"
  echo "✅ Gemini token refreshed"
else
  echo "⚠️  No Gemini credentials — Sage/Blaise/Echo will use z.ai fallbacks"
fi

# === OpenAI Codex (optional) ===
if [ -z "$OPENAI_API_KEY" ]; then
  echo "⚠️  No OpenAI key — Nova will use z.ai fallback"
fi

# Start OpenFang
exec "$(dirname "$0")/../target/release/openfang" start

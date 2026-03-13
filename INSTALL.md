# 🦊 Installation de la Famille OpenFang

Guide complet pour déployer la famille de 34 agents sur n'importe quelle machine (Mac Mini, VPS, laptop).

## Prérequis

- **OS** : Linux (Debian/Ubuntu) ou macOS
- **RAM** : 4 GB minimum (le binaire tourne à ~40 MB)
- **Rust** : 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- **Node.js** : 18+ (pour les scripts de token refresh)
- **Git** : pour cloner le repo

## 1. Cloner et compiler

```bash
git clone https://github.com/bacoco/openfang-fork.git
cd openfang-fork
cargo build --release --workspace --exclude openfang-desktop
```

Le binaire sera dans `target/release/openfang` (~48 MB). Compilation ~10 min.

## 2. Initialiser OpenFang

```bash
./target/release/openfang init
```

Cela crée `~/.openfang/` avec la config de base.

## 3. Variables d'environnement

Créer un fichier `~/.openfang/.env` ou les ajouter à `~/.bashrc` :

```bash
# === OBLIGATOIRE : z.ai (Coding Max plan — gratuit) ===
# Obtenir sur https://z.ai → Settings → API Keys
export ZAI_API_KEY="votre_clé_zai"

# === OPTIONNEL : BytePlus (Coding Plan — $10/mois, ou free packs) ===
# Obtenir sur https://console.byteplus.com → MaaS → API Keys
export BYTEPLUS_API_KEY="votre_clé_byteplus"

# === OPTIONNEL : Google Gemini (gratuit via OAuth) ===
# Nécessite un refresh token obtenu via Gemini CLI ou OpenClaw
# Format JSON avec token, refresh token et project ID
export GEMINI_CLOUDCODE_CREDENTIALS='{"token":"ya29...","refresh":"1//...","projectId":"active-scanner-..."}'
export GEMINI_OAUTH_CLIENT_ID="681255809395-oo8ft2oprdrnp9e3aqf6av3hmdib135j.apps.googleusercontent.com"
export GEMINI_OAUTH_CLIENT_SECRET="voir ci-dessous"

# === OPTIONNEL : OpenAI Codex (gratuit via OAuth) ===
# Nécessite un JWT token obtenu via Codex CLI ou OpenClaw
export OPENAI_API_KEY="eyJ..."
```

### Comment obtenir les tokens OAuth

**Gemini** : Installer Gemini CLI (`npm i -g @anthropic-ai/gemini-cli`), lancer `gemini auth login`, puis extraire le refresh token de `~/.gemini/auth.json`.

**OpenAI Codex** : Installer Codex CLI (`npm i -g @openai/codex`), lancer `codex auth login`, puis extraire le JWT de `~/.codex/auth.json`.

**Sans ces tokens** : Les agents Sage 🦎, Blaise 🧮, Echo 🔮 (Gemini) et Nova 🌟 (Codex) utiliseront automatiquement leurs **fallbacks z.ai** — la famille fonctionne à 100% avec uniquement `ZAI_API_KEY`.

## 4. Installer les agents

Copier les configs d'agents dans `~/.openfang/agents/` :

```bash
cp -r openfang-config/agents/* ~/.openfang/agents/
```

## 5. Démarrer

### Script de démarrage (avec refresh automatique des tokens)

```bash
cp openfang-config/start.sh ~/.openfang/start.sh
chmod +x ~/.openfang/start.sh
# Éditer start.sh pour mettre vos clés et chemins
~/.openfang/start.sh
```

### Démarrage simple (z.ai uniquement)

```bash
export ZAI_API_KEY="votre_clé"
./target/release/openfang start
```

### Spawner tous les agents

```bash
OF=./target/release/openfang
for dir in ~/.openfang/agents/*/; do
  name=$(basename "$dir")
  [ "$name" = "assistant" ] && continue
  $OF agent spawn "$dir/agent.toml" 2>/dev/null
done
echo "$(curl -s http://127.0.0.1:4200/api/agents | python3 -c 'import json,sys;print(len(json.load(sys.stdin)))') agents actifs"
```

## 6. Vérifier

```bash
# Santé du daemon
curl http://127.0.0.1:4200/api/health

# Lister les agents
./target/release/openfang agent list

# Tester un agent
curl -X POST http://127.0.0.1:4200/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"maman","messages":[{"role":"user","content":"Qui es-tu ?"}],"max_tokens":100}'

# Tester TOUS les agents
for agent in maman apex sage nova blaise lyra dante felix henry atlas iris pixel echo spark titan vega golem cine lumen scope nl-henry nl-blaise nl-sage nl-nova nl-atlas nl-iris nl-dante vc-coord vc-scanner vc-analyste vc-redacteur dp-coord dp-redacteur dp-reviewer; do
  result=$(curl -s --max-time 30 -X POST http://127.0.0.1:4200/v1/chat/completions \
    -H "Content-Type: application/json" \
    -d "{\"model\":\"$agent\",\"messages\":[{\"role\":\"user\",\"content\":\"Présente-toi en une phrase.\"}],\"max_tokens\":80}" 2>&1)
  if echo "$result" | grep -q "choices"; then
    echo "✅ $agent"
  else
    echo "❌ $agent"
  fi
done
```

## Architecture de la famille

### 34 agents, 3 familles

| Groupe | Agents | Rôle |
|--------|--------|------|
| 👑 Conseil (8) | maman, apex, sage, nova, blaise, lyra, dante, felix | Délibération et décision |
| 🔧 Spécialistes (12) | henry, atlas, iris, pixel, echo, spark, titan, vega, golem, cine, lumen, scope | Consultants on-demand |
| 📰 Production NL (7) | nl-henry, nl-blaise, nl-sage, nl-nova, nl-atlas, nl-iris, nl-dante | Rédaction newsletters |
| 🔍 Veille Concurrentielle (4) | vc-coord, vc-scanner, vc-analyste, vc-redacteur | Intelligence concurrentielle |
| 📄 Documents Pro (3) | dp-coord, dp-redacteur, dp-reviewer | Génération de documents |

### Providers et modèles

| Provider | Base URL | Modèles | Coût |
|----------|----------|---------|------|
| **z.ai** (OpenAI API) | `api.z.ai/api/coding/paas/v4` | glm-5, glm-4.7, glm-4.6 | Gratuit (Coding Max) |
| **BytePlus** (OpenAI API) | `ark.ap-southeast.bytepluses.com/api/coding/v3` | glm-4.7, kimi-k2-thinking, gpt-oss-120b | $10/mois |
| **Google Gemini** (CloudCode) | `cloudcode-pa.googleapis.com` | gemini-3.1-pro, gemini-3-pro | Gratuit (OAuth) |
| **OpenAI Codex** | `api.openai.com/v1` | gpt-5.3-codex | Gratuit (OAuth) |

### Chaînes de fallback

Chaque agent a 2-3 fallbacks. Si le primary échoue, OpenFang bascule automatiquement :

```
Sage 🦎 : gemini-3.1-pro → glm-4.7 (z.ai) → glm-5 (z.ai)
Nova 🌟 : gpt-5.3-codex → glm-4.7 (z.ai) → glm-5 (z.ai) → gemini-3.1-pro
Apex 🦅 : glm-5 (z.ai) → glm-4.7 (z.ai) → gemini-3.1-pro
```

**Avec uniquement z.ai** : Tous les agents fonctionnent via leurs fallbacks. Aucun provider payant n'est requis.

## Ports

| Port | Service |
|------|---------|
| 4200 | API OpenFang (REST + OpenAI-compatible `/v1/chat/completions`) |

## Fichiers importants

```
~/.openfang/
├── config.toml          # Config générale OpenFang
├── start.sh             # Script de démarrage avec tokens
├── data/
│   └── openfang.db      # Base SQLite (agents, mémoire)
└── agents/
    ├── maman/agent.toml  # Config de chaque agent
    ├── apex/agent.toml
    ├── ...
    └── dp-reviewer/agent.toml
```

## Fork vs upstream

Ce fork ajoute à OpenFang :
- **Driver CloudCode** (`crates/openfang-runtime/src/drivers/cloudcode.rs`) : accès gratuit à Gemini via OAuth
- **Providers z.ai, BytePlus** : ajoutés dans `mod.rs`
- **34 configs d'agents** : la famille complète avec SOULs, modèles, fallbacks

Pour mettre à jour depuis upstream :
```bash
git fetch upstream
git merge upstream/main
# Résoudre les conflits dans mod.rs si nécessaire
cargo build --release --workspace --exclude openfang-desktop
```

## Dépannage

| Problème | Solution |
|----------|----------|
| Agent "processing failed" | Provider saturé → le fallback devrait prendre le relais. Vérifier les env vars. |
| Build échoue sur GTK | Ajouter `--exclude openfang-desktop` au build |
| Token Gemini expiré | Le driver CloudCode le rafraîchit automatiquement via le refresh token |
| Port 4200 occupé | `pkill -f "openfang start"` puis relancer |
| DB corrompue | `rm ~/.openfang/data/openfang.db` puis respawner les agents |

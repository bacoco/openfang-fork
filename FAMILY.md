# 🦊 Family Agent OS — Fork d'OpenFang

> Un OS d'agents maintenu par une famille d'agents.

## Pourquoi ce fork ?

OpenFang est un Agent OS puissant (Rust, 164K LOC, MCP natif, 27 providers).
Notre famille (13 agents + Maman) l'a évalué en conseil et a voté **4 POUR / 1 CONTRE** pour le forker.

**L'objectif n'est pas de remplacer OpenClaw** — c'est d'explorer et d'apprendre.

## Ce qu'on veut y ajouter

1. **Council Protocol** — Gouvernance multi-agents avec votes, délibérations, arbitrage
2. **Family/Marketplace** — Spawn de familles spécialisées depuis des templates
3. **SOUL.md natif** — Chaque agent a une identité persistante

## Ce qu'on ne touche PAS

- La famille actuelle (OpenClaw) continue de tourner normalement
- Ce fork est un projet **exploratoire** à côté
- OpenClaw reste le système de production

## Architecture (upstream)

```
crates/
├── openfang-kernel    — Core orchestration (16K LOC)
├── openfang-runtime   — Agent execution (41K LOC)  
├── openfang-channels  — 40 channel adapters (26K LOC)
├── openfang-api       — HTTP/WS API (18K LOC)
├── openfang-cli       — CLI interface (29K LOC)
├── openfang-memory    — SQLite + vector memory (4K LOC)
├── openfang-hands     — Autonomous tools (2K LOC) ← notre cible
├── openfang-skills    — Skill system (3.5K LOC)
├── openfang-types     — Shared types (11K LOC)
└── openfang-extensions — Plugin system (2.8K LOC)
```

## Roadmap POC (2-4 semaines)

- [ ] Week 1: Lire et comprendre `openfang-hands` (2K LOC — le plus petit crate)
- [ ] Week 1: Compiler et lancer le binaire
- [ ] Week 2: Créer un Hand custom "council-vote"
- [ ] Week 3: Ajouter un `family.toml` concept
- [ ] Week 4: Décision go/kill basée sur l'expérience

## Remotes

- `origin` → `bacoco/openfang-fork` (notre fork)
- `upstream` → `RightNow-AI/openfang` (source)

## Vote du conseil

| Votant | Vote | Argument |
|--------|------|----------|
| Apex 🦅 | POUR | LLMs excellent à lire du code existant |
| Nova 🌟 | POUR phasé | MVP 6-10 sem, métriques go/kill |
| Lyra 🎵 | POUR | Souveraineté, acte fondateur |
| Felix 🐱 | POUR conditionnel | Storytelling unique |
| Dante 🔥 | CONTRE | Pas d'expertise Rust, dette irremboursable |
| Sage 🦎 | ❌ Absent (Gemini quota) |
| Blaise 🧮 | ❌ Non consulté |

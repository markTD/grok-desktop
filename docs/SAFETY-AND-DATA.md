# Safety & data transparency

This document is for people exploring **Grok Desktop** who want a clear picture of **what runs where**, **what leaves the machine**, and **how to stay safer**.

Grok Desktop is a **thin UI**. The coding agent is the official **xAI Grok Build** CLI (`grok`). Most privacy and permission controls live in that CLI and your xAI/SuperGrok account — not only in this app.

---

## Two layers

| Layer | What it is | What it touches |
|-------|------------|-----------------|
| **Grok Desktop** (this app) | Tauri + Svelte shell | Spawns `grok`, shows chat, exports notes under your project |
| **Grok Build CLI** | Official agent from xAI | Models, tools, MCP, sessions under `~/.grok`, network to xAI |

---

## What stays local

- **This app’s UI state** (recent session IDs, last folder, setup-done flag) in browser/local storage for the desktop webview  
- **Session notes you Export** → `<project>/.grok-desktop/notes/*.md` (only when you click Export)  
- **Grok Build session files** → `~/.grok/sessions/<encoded-cwd>/<session-id>/` (conversation logs, plans, etc.)  
- **Grok auth cache** → `~/.grok/auth.json` (managed by the CLI)  
- **Grok config** → `~/.grok/config.toml` (permissions, telemetry toggles, etc.)

Grok Desktop does **not** re-upload your repo through a custom backend of ours. There is no Grok Desktop cloud account.

---

## What can leave your machine (via Grok Build / xAI)

When you chat or run tools through the agent, content needed for the model and tools may be sent to **xAI’s backends** (and any **MCP servers** you enabled). That typically includes:

- Prompts and model responses  
- File contents the agent **reads or edits** as part of tool use  
- Shell command context the agent uses  
- Optional **telemetry / trace** behavior controlled by Grok Build config (see below)

Exact retention and training policies are defined by **xAI / SuperGrok**, not by Grok Desktop. In the official TUI you can inspect status with:

```text
/privacy
```

Also review xAI’s published terms and privacy materials for your plan.

### CLI knobs that matter (Grok Build)

From `~/.grok/config.toml` (examples from the official user guide):

```toml
[features]
telemetry = false          # anonymous usage telemetry master switch

[telemetry]
trace_upload = false       # session/trace uploads (when relevant)
```

Account-level **coding data retention** options may appear in Grok’s privacy UI (`/privacy`) depending on plan.

**Grok Desktop cannot override xAI account policy** — it can only help you see status and choose safer local defaults.

---

## What this app does *not* do

- No separate Grok Desktop telemetry service  
- No hidden “phone home” of your full project as our product  
- No storage of your SuperGrok password (browser OAuth via official CLI)  
- No claim of “air-gapped AI” while Grok Build is connected to xAI  

---

## Safety for explorers (practical)

### Safer defaults in Grok Desktop

| Setting | Safer choice | Why |
|---------|--------------|-----|
| Auto-approve tools | **Off** | You see tool use; fewer surprise writes/shell commands |
| Project folder | **Git repo** | Diff + revert if the agent messes up |
| First sessions | **Learn / Explore / Plan** paths | Prefer read-heavy work before write-heavy |
| Secrets | Don’t put keys in the project | Agents can read files and run shell |

### Permission modes (Grok Build)

The CLI supports modes like `default` (prompt), `dontAsk` (deny unless allowed), and always-approve / bypass styles. Auto-approve in Grok Desktop maps to spawning the agent with **always-approve** — convenient but powerful. Prefer **off** while learning.

### Dangerous folders

Avoid pointing the agent at:

- Your entire home directory  
- Folders with production secrets (`.env` with live keys, cloud credentials)  
- Shared machines without understanding what will run  

Prefer a **dedicated git project** for experiments.

---

## MCP and plugins

If MCP servers or plugins are enabled in `~/.grok`, the agent may call **third-party services** (e.g. Vercel, Sentry). Those have their own data policies. Review enabled plugins before serious work.

---

## Your checklist before first real project

1. [ ] CLI installed and signed in (`grok models` works)  
2. [ ] Working in a **git** folder (or accept risk)  
3. [ ] Auto-approve **off** until you trust the loop  
4. [ ] Know that prompts + tool-read files may go to **xAI**  
5. [ ] Optionally run `/privacy` in the TUI and set retention preferences  
6. [ ] Optionally set `telemetry = false` in `~/.grok/config.toml` if you want less analytics  

---

## Honesty about community concerns

Community tools exist that audit or harden Grok Build upload behavior. Policies and CLI behavior can change by version — **check your installed CLI docs** (`~/.grok/docs/user-guide/`) and xAI’s current terms. This app’s job is transparency and safer local defaults, not guaranteeing zero network data.

---

## Reporting issues

- Grok Desktop: GitHub issues on this repo  
- Grok Build / account privacy: xAI support and official channels  

*Last updated with the public Grok Desktop release. Not legal advice; not affiliated with xAI.*

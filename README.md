# Grok Desktop

**A friendly desktop shell for [xAI Grok Build](https://x.ai/cli)** — so SuperGrok / X Premium+ users can create and learn without living in a TUI.

```
You (UI)  ──ACP──►  grok agent stdio  ──►  tools · MCP · models · auth
```

This app does **not** reimplement the coding agent. It is a thin client around the official `grok` CLI.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## Why

xAI open-sourced the Grok Build harness. The agent is excellent; the default surface is a terminal. **Grok Desktop** is a front door for:

- People who want **Claude Desktop–style** chat over a real coding agent  
- People who are **not great at prompting** (Guided kickoff + multi-step loops)  
- People who want a **learning journal** (export notes) after a session  

---

## Features

| Feature | What it does |
|--------|----------------|
| **Guided setup** | Install CLI → sign in → pick folder → choose how to start |
| **Guided kickoff** | Short interview → strong first prompt + safety rules |
| **Loops** | Explore → plan → implement → verify → **wrap-up summary** |
| **Chat** | Streaming messages, thinking, tools (ACP) |
| **Export notes** | Markdown journal under `.grok-desktop/notes/` |
| **Resume** | Recent sessions via `session/load` |
| **Cancel** | Stop an in-flight turn |
| **CLI / Build monitor** | Version chip, update check, `grok update`, models list |
| **Start paths** | Create / Learn / Fix / Interview without prompt skills |
| **Arsenal prompts** | Plan mode, explore subagents, worktree, review, web/X |
| **Git safety** | Warns when auto-approve + non-git folder |

---

## Requirements

1. **macOS** (primary; Linux should work; Windows best-effort via Tauri)  
2. **Node.js 18+** and **Rust** (for building from source)  
3. **Grok Build CLI** + SuperGrok / supported plan:

   ```bash
   curl -fsSL https://x.ai/cli/install.sh | bash
   grok --version
   grok models   # must succeed when signed in
   ```

---

## Quick start

```bash
git clone <this-repository-url>
cd grok-desktop
npm install
npm run tauri dev
```

First launch opens **Guided setup**. Or: **Browse…** a project → **Kickoff** / **Loops** / **Connect**.

### Production build

```bash
npm run tauri build
```

---

## How to use (60 seconds)

1. Confirm header shows **CLI ready**  
2. Pick a project folder (git recommended)  
3. Either:  
   - **Setup** → walk install/auth if needed  
   - **Kickoff** → answer 4 questions → auto-start  
   - **Loops** → e.g. Ship a feature → goal → watch steps + wrap-up  
4. **Export** → save notes into the project  

---

## Architecture

| Layer | Responsibility |
|-------|----------------|
| Svelte UI | Setup, kickoff, loops, chat, export |
| Tauri / Rust | Spawn `grok`, ACP JSON-RPC, notes export |
| Grok Build CLI | Agent loop, tools, sandbox, MCP, OAuth |

See [docs/PRODUCT.md](docs/PRODUCT.md) and [docs/ROADMAP.md](docs/ROADMAP.md).

---

## Safety & data transparency

**Read [docs/SAFETY-AND-DATA.md](docs/SAFETY-AND-DATA.md)** before serious use.

Short version:

| Topic | Reality |
|-------|---------|
| Grok Desktop cloud? | **No** — local UI only |
| Model / tool traffic | Goes through **official Grok Build CLI → xAI** (and MCP you enable) |
| Sessions on disk | `~/.grok/sessions/` |
| Your exports | Only if you click Export → `.grok-desktop/notes/` |
| Safer exploring | Auto-approve **off**, use a **git** folder, prefer Learn / plan first |

In the app: **Safety** button (or **More → Data & safety**). Account retention: Grok TUI `/privacy`.

---

## Status

Early public release. Expect rough edges. Issues and ideas welcome.

---

## License

MIT for this repository. Grok Build is separate software under xAI’s terms and the [grok-build](https://github.com/xai-org/grok-build) tree (Apache-2.0 for first-party source).

Not affiliated with xAI / SpaceXAI.  

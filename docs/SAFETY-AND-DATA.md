# Safety & data — plain language

**For new users and anyone who cares about privacy.**  
This is the trust document for **Grok Desktop**. Skim the status board first.

---

## Status board (read this first)

| Question | Answer |
|----------|--------|
| Does Grok Desktop have its own cloud that stores my repo? | **No.** |
| Will it zip and upload my **entire** codebase in one shot? | **No default “upload all” button.** |
| Can pieces of my project leave my computer? | **Yes** — when the agent reads/edits files or runs tools for a chat turn, that content can go to **xAI** (via Grok Build). Over a long session that can cover a lot of the tree. |
| Do my chat messages leave the computer? | **Yes** — to xAI so the model can answer (same family of product as the official Grok Build TUI). |
| Where is history stored on my Mac? | **Locally** under `~/.grok/sessions/` (official CLI). |
| When do notes leave the project folder? | **Only if you click Export** → `.grok-desktop/notes/`. |
| Who controls training / retention? | **xAI / your SuperGrok plan** — check **`/privacy`** inside the official `grok` TUI. This app cannot override that. |

---

## Two products, one UI

```
You → Grok Desktop (open source shell) → Grok Build CLI (xAI) → xAI models (+ any MCP you enabled)
```

| Layer | Job |
|-------|-----|
| **Grok Desktop** | Friendly window, loops, safety screen, safer defaults |
| **Grok Build** | Real agent harness (tools, permissions, sessions, updates) |
| **You** | Folder choice, git, secrets hygiene, when to turn on auto-approve |

---

## Harness & guardrails

### Layer 1 — Grok Desktop
- No separate backend that receives your repo  
- Auto-approve **off** by default  
- Warns on **non-git + auto-approve**  
- First-run **Safety / Data status** screen  
- Clear status board in-app (**Safety** button)

**Limit:** Once you Connect, model traffic is handled by the official CLI and xAI. We cannot promise “zero network.”

### Layer 2 — Grok Build CLI (xAI harness)
- File/shell/web tools, plan mode, subagents, sandbox options, MCP  
- Config: `~/.grok/config.toml` (`telemetry`, permission modes, …)  
- Account privacy: TUI command **`/privacy`**

**Limit:** Designed as a capable online coding agent.

### Layer 3 — You
- Use a **git project**, not `$HOME`  
- Prefer **Learn / plan / explore** first  
- Keep live secrets out of the tree  
- Review plugins/MCP before serious work  

---

## What stays local vs what can leave

### Stays on your machine
- UI prefs (last folder, setup flags)  
- `~/.grok/sessions/…` conversation stores  
- `~/.grok/auth.json` (CLI-managed login cache)  
- Exported notes (only if you export)

### Can leave (via Grok Build → xAI / MCP)
- Prompts and model replies  
- File contents the agent **opens or changes**  
- Tool/shell output fed back to the model  
- Optional telemetry/traces if enabled in CLI config  
- Data to third parties if you enabled MCP/plugins  

---

## Safer defaults checklist

1. Auto-approve **off** while learning  
2. Project is a **git** repo  
3. Folder is a **dedicated project**, not home  
4. Read in-app **Safety** status board  
5. Run TUI **`/privacy`** for account retention  
6. Optionally set `telemetry = false` under `[features]` in `~/.grok/config.toml`  

In the app: **Safe explore mode** forces auto-approve off and points you at a read-heavy path.

---

## Honest limits

- Community concerns exist about coding agents and uploads; CLI behavior can change by version.  
- Always re-check **your** CLI version, `~/.grok/docs`, and xAI’s current terms.  
- This document is **not legal advice** and **not affiliated with xAI**.

---

## In the app

| Control | Where |
|---------|--------|
| Status board + harness explanation | **Safety** button |
| Safe explore | Safety panel |
| Auto-approve / model / export | **More** |
| Build version / update | Version chip → Build monitor |

*Last updated for the public Grok Desktop trust-center UI.*

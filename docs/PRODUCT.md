# Grok Desktop — product brief

**Working title:** Grok Desktop  
**Stack:** Tauri 2 + SvelteKit (TS) + official Grok Build CLI via ACP  
**License:** MIT (app); Grok Build remains under xAI / Apache-2.0 as upstream

## Vision

Make Grok Build **accessible to everyone who wants to vibe-code** — not only people who live in a TUI. Claude Desktop–style shell, plus a **guided kickoff** so you don’t need to be good at prompting.

xAI open-sourcing the agent harness is extraordinary; this client is how we put a friendly front door on it.

## Architecture

```
┌─────────────────────────────────────┐
│  Grok Desktop (this repo)           │  chat, onboarding, kickoff, permissions
└─────────────────┬───────────────────┘
                  │  ACP JSON-RPC
┌─────────────────▼───────────────────┐
│  grok agent stdio                   │  tools, MCP, sandbox, models, auth
└─────────────────────────────────────┘
```

## Features (current)

| Area | Status |
|------|--------|
| CLI probe (path, version, soft auth) | Done |
| ACP connect / prompt / stream | Done |
| Folder picker | Done |
| Markdown assistant replies | Done |
| Collapsible thinking | Done |
| Auto-approve toggle | Done |
| Permission request modal (when agent asks) | Done |
| Session resume (`session/load` + recent list) | Done |
| First-run tour + help tips | Done |
| Guided kickoff (interview → rules + starter prompt) | Done |
| Multi-step orchestration loops (explore→plan→…) | Done |
| Cancel in-flight turn | Done |
| Token usage readout (session-ish) | Done |
| Model / effort on connect | Done |
| Parallel multi-agent arena UI | Future |

## Guided kickoff (prompt builder)

A short interview collects:

1. Goal (plain English)  
2. Kind of work (existing / greenfield / fix / learn)  
3. Risk (careful / balanced / fast+auto-approve) + plan-first  
4. Optional constraints  

We derive:

- `session/new` `_meta.rules`  
- A strong **starter prompt**  
- Safety defaults  

No model call is required for the interview — pure product logic so it stays free and fast.

## Non-goals (for now)

- Reimplementing agent tools / sandbox / MCP host  
- Linking crates from `xai-org/grok-build`  
- Full multi-provider cowork suite  

## Auth & install

- SuperGrok / supported xAI subscription (as required by Grok Build)  
- CLI: `curl -fsSL https://x.ai/cli/install.sh | bash`  
- Optional: `GROK_BINARY=/path/to/grok`

## Roadmap

See [ROADMAP.md](ROADMAP.md) for ordered steps (reliability → finish-the-job → UI de-clutter → how-to → starter packs). Starter project templates are parked until the core path is solid.

## Portfolio framing

> Desktop ACP client for xAI Grok Build (Tauri/Svelte): guided onboarding, streaming tool traces, session resume, and interactive permission UX over the Agent Client Protocol — so anyone can use a production coding agent without living in a TUI.

## References

- [xai-org/grok-build](https://github.com/xai-org/grok-build)  
- [Agent Client Protocol](https://agentclientprotocol.com)  
- Install: https://x.ai/cli  

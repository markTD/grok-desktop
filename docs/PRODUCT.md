# Grok Desktop — product brief

**Working title:** Grok Desktop  
**Stack:** Tauri 2 + SvelteKit (TS) + official Grok Build CLI via ACP  
**License:** MIT (app); Grok Build remains under xAI / Apache-2.0 as upstream

## Problem

Grok Build is a strong coding agent (TUI + headless + ACP) with no first-party desktop GUI. Claude Desktop–style UX is what many people want: sessions, tool visibility, permission prompts — without rebuilding the agent.

## Solution

A **thin desktop client** that:

1. Locates and probes the local `grok` binary  
2. Speaks **Agent Client Protocol** to `grok agent stdio`  
3. Renders streaming text, thoughts, tool calls, plans  
4. Surfaces permission approve/deny in the UI  

```
┌─────────────────────────────────────┐
│  Grok Desktop (this repo)           │  chat, tools, permissions, sessions
└─────────────────┬───────────────────┘
                  │  ACP JSON-RPC
┌─────────────────▼───────────────────┐
│  grok agent stdio                   │  tools, MCP, sandbox, models, auth
└─────────────────────────────────────┘
```

## Non-goals (v1)

- Reimplementing agent tools, sandbox, or MCP host  
- Linking crates from `xai-org/grok-build`  
- Multi-provider / Ollama / generic “cowork suite”  
- Forking or contributing to the closed-contribution monorepo  

## MVP checklist

- [x] Project scaffold + git  
- [x] `grok` binary resolve + version + soft auth (`grok models`)  
- [ ] ACP stdio spawn + `initialize` + `session/new`  
- [ ] Stream `session/update` into UI  
- [ ] Permission requests  
- [ ] Project folder picker  
- [ ] README demo GIF + architecture  

## Auth & install

Users need:

- SuperGrok / supported xAI subscription (as required by Grok Build)  
- CLI: `curl -fsSL https://x.ai/cli/install.sh | bash`  
- Optional override: `GROK_BINARY=/path/to/grok`

This app does **not** store API keys for the agent path; it reuses CLI OAuth (`~/.grok`).

## Portfolio framing

> Desktop ACP client for xAI Grok Build (Tauri/Svelte): process-managed agent sessions, streaming tool traces, and interactive permission UX over the Agent Client Protocol.

## References

- [xai-org/grok-build](https://github.com/xai-org/grok-build)  
- [Agent Client Protocol](https://agentclientprotocol.com)  
- Grok agent mode: `grok agent stdio` / `grok agent serve`  
- Install: https://x.ai/cli  

# Grok Desktop

A friendly **desktop shell for [Grok Build](https://x.ai/cli)** — Claude Desktop–style UX over the official coding agent, via the [Agent Client Protocol (ACP)](https://agentclientprotocol.com).

Built for people who want to **vibe-code with Grok** without mastering a TUI or prompt engineering.

```
Grok Desktop (UI)  ──ACP──►  grok agent stdio  ──►  tools / MCP / models / auth
```

## Why this exists

xAI open-sourced the Grok Build harness. The agent is excellent; the default surface is a terminal. **Grok Desktop** is a thin client so more people can:

- Pick a project folder  
- Start with a **Guided kickoff** (short interview → strong first prompt)  
- See thinking, tools, and markdown replies  
- Resume sessions  
- Optionally auto-approve tools for fast loops  

## Features

- **Guided kickoff** — interview → rules + starter prompt (prompt builder)  
- **Orchestration loops** — multi-step explore → plan → implement → verify (and more)  
- **Connect / Disconnect** ACP sessions + **Cancel** in-flight turns  
- **Resume** recent sessions (`session/load`)  
- **Browse…** folder picker  
- Streaming **message / thought / tool** updates (tool cards merge by id)  
- **Markdown** assistant output  
- **Token usage** summary from turn metadata  
- **Model / effort** selectors at connect  
- **Auto-approve** toggle + permission modal when the agent asks  
- First-run **tour** + contextual **?** help tips  

## Prerequisites

1. **Node.js** 18+ and **Rust** (stable)  
2. **Grok Build CLI** signed in:

   ```bash
   curl -fsSL https://x.ai/cli/install.sh | bash
   grok --version
   grok models
   ```

3. Optional: `export GROK_BINARY=/path/to/grok`

## Develop

```bash
cd grok-desktop
npm install
npm run tauri dev
```

## How to use (60 seconds)

1. Confirm header shows CLI **Ready** / authenticated  
2. **Browse…** to a project (preferably a git repo)  
3. Either:  
   - **Guided kickoff** → answer 4 short questions → launch, or  
   - **Connect** → type freely  
4. Watch tools + replies stream in  

## Architecture

| Layer | Responsibility |
|-------|----------------|
| Svelte UI | Onboarding, kickoff, chat, permissions |
| Tauri / Rust | Spawn `grok`, ACP JSON-RPC, process lifecycle |
| Grok Build CLI | Agent loop, tools, sandbox, MCP, OAuth |

See [docs/PRODUCT.md](docs/PRODUCT.md).

## License

MIT for this repository. Grok Build itself is separate software (see xAI terms and [grok-build](https://github.com/xai-org/grok-build)).

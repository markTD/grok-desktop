# Grok Desktop

A macOS-friendly **desktop shell for [Grok Build](https://x.ai/cli)** — Claude Desktop–style UX over the official agent, via the [Agent Client Protocol (ACP)](https://agentclientprotocol.com).

```
Grok Desktop (UI)  ──ACP──►  grok agent stdio  ──►  tools / MCP / models / auth
```

This app does **not** reimplement the coding agent. It is a thin client around the `grok` CLI you already install from xAI.

## Status

**Phase 0 — bootstrapped**

- [x] Tauri 2 + SvelteKit (TypeScript) scaffold  
- [x] Locate `grok` (`PATH`, `~/.grok/bin/grok`, or `GROK_BINARY`)  
- [x] Soft readiness check (`grok --version`, `grok models`)  
- [ ] ACP session + streaming chat UI (next)

See [docs/PRODUCT.md](docs/PRODUCT.md) for scope, non-goals, and MVP checklist.

## Prerequisites

1. **Node.js** 18+ and **Rust** (stable)  
2. **Grok Build CLI** and a signed-in session:

   ```bash
   curl -fsSL https://x.ai/cli/install.sh | bash
   grok --version
   grok models    # should succeed when authenticated
   ```

3. Optional: `export GROK_BINARY=/path/to/grok`

## Develop

```bash
cd grok-desktop
npm install
npm run tauri dev
```

Frontend only (no native probe): `npm run dev`

## Architecture

| Layer | Responsibility |
|-------|----------------|
| **Svelte UI** | Status, chat, tool cards, permissions (upcoming) |
| **Tauri / Rust** | Spawn `grok`, ACP JSON-RPC, process lifecycle |
| **Grok Build CLI** | Agent loop, tools, sandbox, MCP, OAuth |

## License

MIT for this repository. Grok Build itself is separate software (see xAI terms and the [grok-build](https://github.com/xai-org/grok-build) Apache-2.0 tree).

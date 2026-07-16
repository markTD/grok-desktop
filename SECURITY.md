# Security

## Reporting issues

If you find a vulnerability in **Grok Desktop** (this repo), please open a private report via GitHub Security Advisories if available, or open an issue without including exploit details that put others at risk.

This project is a thin UI over the official **xAI Grok Build** CLI. Issues in Grok Build, the xAI API, or account privacy should go to **xAI**, not this repository.

## What this project does not store

- No API keys are required or stored by Grok Desktop for the ACP path (auth is the CLI’s `~/.grok` session).
- Do not commit `.env`, tokens, or private keys. See `.gitignore`.

## What users should know

- Coding agents can send project content to the model provider when tools run. See [docs/SAFETY-AND-DATA.md](docs/SAFETY-AND-DATA.md).
- **Quality packs** (ship-ready, security audit, pre-share) are AI-assisted helpers, **not** formal security audits or compliance certifications.
- Before publishing any project: run `git status` / `git diff`, search for secrets, and rotate anything that was ever committed.

## Secrets hygiene for contributors

```bash
# Before push
git status
rg -n -i 'api[_-]?key|secret|password|token|sk-|ghp_|BEGIN ' --glob '!.git' --glob '!node_modules' --glob '!target'
```

Never paste live credentials into issues, commits, or chat logs.

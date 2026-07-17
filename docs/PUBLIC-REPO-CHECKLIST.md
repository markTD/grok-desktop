# Public repo checklist (Grok Desktop)

Use before promoting this repository or any project built with the app.

## This repository (Grok Desktop)

- [x] No `.env` or credential files tracked  
- [x] Hardcoded personal home path removed from default cwd  
- [x] `.gitignore` covers env, keys, `.grok-desktop/` notes  
- [x] `SECURITY.md` present  
- [x] Safety / data docs present  
- [x] Spot-check `git log` / diffs for accidental paths or tokens  
- [x] History scrubbed of local home paths (`/Users/…`)  
- [x] No username fingerprint in app id, authors, license, or docs  
- [x] Demo GIF/MP4 shipped under `assets/demo/`  

```bash
rg -n -i 'api[_-]?key|secret|password|sk-|ghp_|BEGIN |/Users/' \
  --glob '!.git' --glob '!node_modules' --glob '!target' --glob '!package-lock.json' .
```

## Any project you built with packs

1. Run **Packs → Pre-share check** or **Ship-ready check**  
2. `git status` and `git diff`  
3. Search for secrets (command above)  
4. Confirm no `.env` in git: `git ls-files | rg -i 'env|secret|credential|pem|key$'`  
5. Treat quality-pack verdicts as **advice with caveats**, not a certificate  

## Caveats we state in the product

- AI review can miss issues or be wrong  
- User is responsible for what they ship  
- Rotate secrets out-of-band; never paste full secrets into chat  
- xAI does not host public websites  

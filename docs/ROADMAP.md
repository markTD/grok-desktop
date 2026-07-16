# Grok Desktop — focused roadmap

**Audience:** X Premium+ / SuperGrok users who want to *create* or *learn* without living in a TUI.  
**Principle:** functionality → clarity of steps → UI polish → teaching content.  
**Not now:** every idea that pours out (templates for “website / voice / maximize X”) stays parked until the core path is rock solid.

---

## North star (one sentence)

A Premium+ user can open Grok Desktop, pick a folder, run a guided path, and finish something real — with optional learning along the way.

---

## Ordered steps (do these in order)

### Step A — Reliability of the core path *(now)*
What must never feel broken:

1. CLI status always honest and visible (binary + auth)  
2. Connect / loop / free chat all work  
3. **Loop end is unmistakable** (done banner + panel state)  
4. Cancel stops a run  
5. Resume recent sessions  

**Exit criteria:** You can demo A→finish without explaining “ignore that button.”

### Step B — Finish-the-job functionality
Still product, not paint:

1. After a loop: **one summary prompt** (“what shipped / how to verify”) if the last step was thin  
2. Export session notes (markdown file in project) for portfolio / learning journal  
3. Safer defaults for non-git folders (warn before auto-approve)  
4. Optional: open finished plan file / session folder  

**Exit criteria:** User leaves with an artifact, not only chat scrollback.

### Step C — UI de-clutter *(after B)*
Toolbar is getting busy. Collapse into:

- **Primary:** Project · Connect/Disconnect · Composer  
- **Secondary drawer:** Kickoff · Loops · Effort/Model · Logs · Tour  

**Exit criteria:** First 10 seconds look simple; power options one click away.

### Step D — How-to / teaching layer
In-app, not a separate product:

1. Short “paths” copy: *Create something* vs *Learn this repo* vs *Fix a bug* — **done**  
2. Tooltips / trust center — **done** (Safety)  
3. Optional **Explain mode** toggle — **done** (toolbar + More; session rules + per-turn nudge)  
4. README + 60s GIF for GitHub / portfolio — **GIF still open**  

**Exit criteria:** A Premium+ friend can use it without you on a call.

### Step E — Starter project packs
In-app **Packs** (content + loop presets). xAI does **not** host public sites — packs document local run + Pages/Netlify/Vercel.

| Pack | Status |
|------|--------|
| Personal website | Shipped |
| Product landing page | Shipped |
| X / social content kit | Shipped |
| Local utility app | Shipped |
| Learn this codebase | Shipped |
| Fix something broken | Shipped |
| Voice / media (Imagine) | Future |
| Full SaaS + auth + DB | Out of scope for packs (needs real hosting stack) |

---

## Explicit non-goals (for a while)

- Parallel multi-agent arena UI  
- Replacing Grok Build / forking the monorepo  
- Full multi-provider cowork suite  
- Pixel-perfect Claude Desktop clone before A–B work  

---

## How we use *you* (and me) efficiently

| Mode | When |
|------|------|
| **Functionality sprint** | One vertical slice, ship, you try it |
| **Plan only** | When ideas multiply — update this file first |
| **Polish sprint** | Only after A–B exit criteria |

When ideas pour out: **add a line under Step E / parking lot**, don’t change A–B mid-flight.

### Parking lot
- Common project templates (site, voice, X creator packs)  
- Marketplace of community loops  
- Multi-window / multi-session dashboard  

---

### Build monitor (functionality — keep current)

Track and update the Grok Build CLI itself from the desktop shell:

1. Version chip + `grok update --check`  
2. One-click `grok update` when behind  
3. Models cache list (for model picker)  
4. Session / plan folder reveal  

---

## Current position

| Step | Status |
|------|--------|
| A–C | Done |
| D | Paths + Safety + **Explain mode**; GIF optional |
| Build monitor | Done |
| Trust / safety | Done (keep iterating if users confuse) |
| E | First 6 packs shipped; voice/media later |
| Optional next | Demo GIF · more packs · use-testing |

Last updated: 2026-07-15

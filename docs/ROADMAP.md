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

1. Short “paths” copy: *Create something* vs *Learn this repo* vs *Fix a bug*  
2. Tooltips that explain *why* a step exists (explore saves tokens, etc.)  
3. Optional “explain mode” toggle (already partly in learn loop)  
4. README + 60s GIF for GitHub / portfolio  

**Exit criteria:** A Premium+ friend can use it without you on a call.

### Step E — Starter project packs *(parked until D)*
Your good idea — **after** the path is simple:

| Pack | Outcome |
|------|---------|
| Personal site | Static site scaffold + deploy notes |
| “Ship on X” | Thread/post helper using Grok/X strengths |
| Voice / media | Thin starter around Grok media tools |
| Learn-to-code | Toy repo + learn loop presets |

These are **content + loop presets**, not a new architecture. Build one pack well before five.

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
| A | Done |
| B | B1–B3 + session/plan open |
| C | Primary toolbar + More drawer |
| D | Start paths + arsenal + trust center (data status / harness) |
| Build monitor | Shipped |
| Trust / safety UX | Status board + guardrails tabs (iterate on clarity) |
| E | Parked |

Last updated: 2026-07-15

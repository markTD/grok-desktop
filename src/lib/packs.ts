/**
 * Starter packs — what most Premium+ / vibe-coders actually want to do.
 *
 * Hosting note: xAI does NOT host public websites. Packs that need a public URL
 * deploy to GitHub Pages / Vercel / Netlify (or run local only).
 *
 * Quality packs reduce risk; they do not guarantee perfect security or design.
 */

/** Shown in UI and injected into quality pack session rules. */
export const QUALITY_PACK_CAVEATS = [
  "Caveats (be honest with the user):",
  "- This review is AI-assisted. It can miss issues or be wrong. You are still responsible for what you ship.",
  "- Always run git diff (or review changes) before publishing or posting a link.",
  "- If secrets were ever committed, rotate them outside this chat — do not paste full secret values into the conversation.",
  "- “Ship-ready” means “much safer and less embarrassing,” not “certified secure” or “enterprise audited.”",
].join("\n");

export type PackCategory = "build" | "quality" | "content";

export type StarterPack = {
  id: string;
  title: string;
  blurb: string;
  category: PackCategory;
  /** Why this pack is popular */
  why: string;
  /** Where the result lives when “done” */
  hosting: string;
  /** Prefer explain mode */
  explainMode: boolean;
  preferAutoApprove: boolean;
  /** Session rules for connect */
  rules: string;
  /**
   * Goal template. Use {{topic}} for optional user fill-in.
   */
  goalTemplate: string;
  /** Placeholder for the goal field */
  goalPlaceholder: string;
  /** Which orchestration loop to run, or null for single-shot */
  loopId: string | null;
  /** Extra plain-language caveat for the packs UI */
  caveat?: string;
};

export const PACK_CATEGORY_LABELS: Record<PackCategory, string> = {
  build: "Build",
  quality: "Safe & proud to show",
  content: "Content & media",
};

export const STARTER_PACKS: StarterPack[] = [
  // ── Build ──────────────────────────────────────────────────────────────
  {
    id: "personal-site",
    title: "Personal website",
    category: "build",
    blurb: "Clean one-pager or multi-page site about you / your brand.",
    why: "Most common first win for non-engineers using coding agents.",
    hosting:
      "Local first. Public URL: GitHub Pages, Netlify, or Vercel — not xAI hosting.",
    explainMode: true,
    preferAutoApprove: false,
    rules: [
      "You are building a personal website in this folder.",
      "Prefer simple static HTML/CSS/JS or a minimal Vite/Svelte/static setup — no heavy backend unless asked.",
      "Mobile-friendly, accessible, fast. No fake stock photos required; use CSS/SVG if needed.",
      "xAI does not host public sites — explain local open + optional Pages/Netlify/Vercel.",
      "Keep secrets out of the repo. No paid APIs required for v1.",
    ].join("\n"),
    goalTemplate:
      "Build a polished personal website for me. Sections: about, what I do, contact. Topic/name if relevant: {{topic}}. Make it look modern and shippable today.",
    goalPlaceholder: "Your name or brand (optional)",
    loopId: "ship-feature",
  },
  {
    id: "landing",
    title: "Product landing page",
    category: "build",
    blurb: "Marketing page for an idea, SaaS, or side project.",
    why: "People want something they can show friends/investors quickly.",
    hosting:
      "Static site locally; public deploy via Vercel/Netlify/GitHub Pages — not xAI.",
    explainMode: true,
    preferAutoApprove: false,
    rules: [
      "Build a single marketing landing page (hero, benefits, how it works, CTA, FAQ).",
      "Static HTML/CSS/JS or lightweight framework. No real payments backend unless asked.",
      "Explain local preview and optional free static deploy.",
    ].join("\n"),
    goalTemplate:
      "Create a product landing page for: {{topic}}. Include hero, 3 benefits, how it works, and a clear call-to-action. Look professional.",
    goalPlaceholder: "Product name + one-sentence pitch",
    loopId: "ship-feature",
  },
  {
    id: "local-tool",
    title: "Local utility app",
    category: "build",
    blurb: "Small tool that runs on your machine (CLI or simple UI).",
    why: "Practical automation without needing a public server.",
    hosting: "Runs on your computer only. No public hosting required.",
    explainMode: true,
    preferAutoApprove: false,
    rules: [
      "Build a small local utility the user can run on their machine.",
      "Prefer Python or Node with a clear README: install + run.",
      "No cloud deploy required. Keep dependencies minimal.",
    ].join("\n"),
    goalTemplate:
      "Build a simple local tool that: {{topic}}. Include README with run instructions. Keep it small and working.",
    goalPlaceholder: "e.g. rename photos, convert CSV, daily notes CLI",
    loopId: "ship-feature",
  },
  {
    id: "learn-first",
    title: "Learn this codebase",
    category: "build",
    blurb: "Map and explain an existing project in plain English.",
    why: "Half of agent use is “I inherited a repo / I don’t understand it.”",
    hosting: "No deploy — learning notes in chat (Export optional).",
    explainMode: true,
    preferAutoApprove: true,
    rules: [
      "Teaching mode: explain like a patient mentor. Prefer read-only exploration.",
      "Use explore subagents for broad search. Avoid large edits unless asked.",
    ].join("\n"),
    goalTemplate:
      "Help me understand this codebase. Focus: {{topic}}. Map structure, entry points, and how to run it.",
    goalPlaceholder: "What you care about (or leave blank for whole project)",
    loopId: "learn-codebase",
  },
  {
    id: "bug-fix",
    title: "Fix something broken",
    category: "build",
    blurb: "Diagnose and fix a bug with a careful loop.",
    why: "Second most common real task after “build me a site.”",
    hosting: "N/A — code change in this repo; use git to review.",
    explainMode: true,
    preferAutoApprove: false,
    rules: [
      "Fix bugs with minimal diffs. Prefer git-friendly changes.",
      "Explain cause and how to verify the fix.",
    ].join("\n"),
    goalTemplate: "Bug / broken behavior: {{topic}}. Reproduce, find root cause, fix, verify.",
    goalPlaceholder: "What is broken? (error text or expected vs actual)",
    loopId: "fix-bug",
  },

  // ── Quality: safe & not embarrassing ───────────────────────────────────
  {
    id: "ship-ready",
    title: "Ship-ready check",
    category: "quality",
    blurb: "Security + design + quality so you won’t be embarrassed to show it.",
    why: "AI-built apps often “work” but leak secrets, look amateur, or crash on demo day.",
    hosting: "Improves this project. Use git diff before you publish anything.",
    caveat:
      "AI review can miss things. Not a formal security audit. You still own what you ship.",
    explainMode: true,
    preferAutoApprove: false,
    rules: [
      "Mission: make this project safer, solid, and presentable.",
      "Never commit secrets. Flag .env, keys, tokens immediately — never paste full secret values.",
      "Prefer small high-impact fixes. Be honest if it is not demo-ready.",
      QUALITY_PACK_CAVEATS,
    ].join("\n"),
    goalTemplate:
      "Run a full ship-ready pass on this project. Focus area if any: {{topic}}. Priority: security first, then embarrassing UX, then code quality. Leave me a demo checklist. Be honest about residual risk.",
    goalPlaceholder: "Optional focus (e.g. login page, whole app)",
    loopId: "ship-ready",
  },
  {
    id: "security-audit",
    title: "Security audit",
    category: "quality",
    blurb: "Find secrets, unsafe patterns, and risky defaults — then fix the worst.",
    why: "Vibe-coded apps often ship with keys in git and open endpoints.",
    hosting: "Report + patches in this repo. Rotate any real keys you find outside the agent.",
    caveat:
      "Not a penetration test or compliance cert. Rotate any real secrets yourself.",
    explainMode: true,
    preferAutoApprove: false,
    rules: [
      "Security-first auditor. Prefer read-only until Critical/High findings are listed.",
      "If secrets are found, instruct the user to rotate them (do not paste full secrets back).",
      "After the audit, fix Critical/High issues with minimal diffs when safe.",
      QUALITY_PACK_CAVEATS,
    ].join("\n"),
    goalTemplate:
      "Security audit this project. Focus: {{topic}}. List Critical/High/Medium/Low with paths, then fix Critical and High if safe. Never echo full secret values. State residual risks clearly.",
    goalPlaceholder: "Optional: auth, API routes, env files…",
    loopId: "ship-ready",
  },
  {
    id: "design-polish",
    title: "Design polish",
    category: "quality",
    blurb: "Make UI/copy look intentional — not placeholder or cringey.",
    why: "“It works but looks like 2009” kills trust faster than missing features.",
    hosting: "UI/copy changes in this project; preview locally.",
    caveat: "Taste is subjective. Review the UI yourself before you share it.",
    explainMode: true,
    preferAutoApprove: false,
    rules: [
      "You are a product designer + frontend polisher.",
      "Fix: spacing, typography, contrast, mobile layout, empty/error states, awkward copy, lorem/TODO text.",
      "Keep brand simple and modern. No stock photo spam. Prefer consistency over novelty.",
      "Do not rewrite backend unless required for a UI bug.",
      QUALITY_PACK_CAVEATS,
    ].join("\n"),
    goalTemplate:
      "Design polish this project so I would not be embarrassed to show a friend. Focus: {{topic}}. Fix the top visual and copy issues; keep it professional.",
    goalPlaceholder: "Optional: landing, settings, whole UI",
    loopId: "ship-feature",
  },
  {
    id: "pre-share",
    title: "Pre-share / pre-post check",
    category: "quality",
    blurb: "Last look before you link it on X or send to someone.",
    why: "People share too early and get roasted for typos, broken mobile, or open .env.",
    hosting: "Checklist + fixes. You still decide when to post.",
    caveat:
      "Verdict is advisory. Double-check git status and secrets yourself before posting a link.",
    explainMode: true,
    preferAutoApprove: false,
    rules: [
      "Act like a blunt friend reviewing before a public share.",
      "Checklist: secrets, broken links, mobile, typos, README, how to run, obvious bugs, cringey copy.",
      "Fix quick wins. Be honest if it should not be shared yet.",
      "End with: SHARE / SHARE WITH CAVEATS / DO NOT SHARE YET and why.",
      QUALITY_PACK_CAVEATS,
    ].join("\n"),
    goalTemplate:
      "I am about to share this project (or post about it). Pre-share review. Context: {{topic}}. Fix quick wins; give a clear SHARE verdict with caveats.",
    goalPlaceholder: "Where will you share? (X, friend, client…)",
    loopId: "ship-ready",
  },

  // ── Content ────────────────────────────────────────────────────────────
  {
    id: "x-presence",
    title: "X / social content kit",
    category: "content",
    blurb: "Thread drafts, bio, and a simple content calendar for X.",
    why: "Premium+ users live on X; Grok’s strength is X-aware content.",
    hosting: "Markdown in this project. Post manually on X.",
    explainMode: true,
    preferAutoApprove: true,
    rules: [
      "You help with X (Twitter) presence: bio options, pinned-post ideas, thread drafts, content calendar.",
      "Use web/X search when available. Be direct and on-brand. Avoid cringe engagement bait.",
      "Write files into /content or similar. Do not post unless explicitly asked and tools allow.",
    ].join("\n"),
    goalTemplate:
      "Build an X content kit for: {{topic}}. Deliver: 3 bio variants, 1 pinned-post draft, 5 thread outlines, and a 2-week content calendar in markdown.",
    goalPlaceholder: "Your niche / account theme",
    loopId: null,
  },
  {
    id: "media-kit",
    title: "Media / Imagine kit",
    category: "content",
    blurb: "Image/video prompts and asset plan for a brand or launch (via Grok media tools if available).",
    why: "People want visuals that match the product — not random stock.",
    hosting:
      "Prompts + any generated assets in the project. Media tools depend on your Grok plan/CLI capabilities.",
    explainMode: true,
    preferAutoApprove: true,
    rules: [
      "Help create a small media kit: brand visual direction, image prompts, optional video prompts.",
      "If image/video generation tools exist in this environment, use them carefully and save assets into /assets or /media.",
      "If tools are unavailable, deliver excellent prompts and a shot list the user can run elsewhere.",
      "Keep outputs tasteful and on-brand; avoid anything that would embarrass a professional share.",
    ].join("\n"),
    goalTemplate:
      "Create a media kit for: {{topic}}. Include visual direction, 5 image prompts, 2 short video ideas, and file naming for assets. Generate images/videos only if tools are available.",
    goalPlaceholder: "Brand or product to visualize",
    loopId: null,
  },
];

export function fillGoal(template: string, topic: string): string {
  const t = topic.trim() || "general / as appropriate for this folder";
  return template.replace(/\{\{topic\}\}/g, t);
}

export function getPack(id: string): StarterPack | undefined {
  return STARTER_PACKS.find((p) => p.id === id);
}

export function packsByCategory(): { category: PackCategory; label: string; packs: StarterPack[] }[] {
  const order: PackCategory[] = ["build", "quality", "content"];
  return order.map((category) => ({
    category,
    label: PACK_CATEGORY_LABELS[category],
    packs: STARTER_PACKS.filter((p) => p.category === category),
  }));
}

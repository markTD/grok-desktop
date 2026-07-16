/**
 * Starter packs — what most Premium+ / vibe-coders actually want to do.
 *
 * Hosting note: xAI does NOT host public websites. Packs that need a public URL
 * deploy to GitHub Pages / Vercel / Netlify (or run local only).
 */

export type StarterPack = {
  id: string;
  title: string;
  blurb: string;
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
   * Goal template. Empty string means ask user in the UI.
   * Use {{topic}} for optional user fill-in.
   */
  goalTemplate: string;
  /** Placeholder for the goal field */
  goalPlaceholder: string;
  /** Which orchestration loop to run, or null for single-shot goal as first prompt */
  loopId: string | null;
};

export const STARTER_PACKS: StarterPack[] = [
  {
    id: "personal-site",
    title: "Personal website",
    blurb: "Clean one-pager or multi-page site about you / your brand.",
    why: "Most common first win for non-engineers using coding agents.",
    hosting:
      "Local first (open index.html or a simple static server). Public URL: GitHub Pages, Netlify, or Vercel — not xAI hosting.",
    explainMode: true,
    preferAutoApprove: false,
    rules: [
      "You are building a personal website in this folder.",
      "Prefer simple static HTML/CSS/JS or a minimal Vite/Svelte/static setup — no heavy backend unless asked.",
      "Mobile-friendly, accessible, fast. No fake stock photos required; use CSS/SVG if needed.",
      "xAI does not host public sites — when done, explain how to open locally and how to deploy to GitHub Pages or Netlify/Vercel if they want a public URL.",
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
    blurb: "Marketing page for an idea, SaaS, or side project.",
    why: "People want something they can show friends/investors quickly.",
    hosting:
      "Static site locally; public deploy via Vercel/Netlify/GitHub Pages — not hosted by xAI.",
    explainMode: true,
    preferAutoApprove: false,
    rules: [
      "Build a single marketing landing page (hero, benefits, how it works, CTA, FAQ).",
      "Static HTML/CSS/JS or lightweight framework. No real payments backend unless asked.",
      "Explain local preview and optional free static deploy (Vercel/Netlify/Pages).",
    ].join("\n"),
    goalTemplate:
      "Create a product landing page for: {{topic}}. Include hero, 3 benefits, how it works, and a clear call-to-action. Look professional.",
    goalPlaceholder: "Product name + one-sentence pitch",
    loopId: "ship-feature",
  },
  {
    id: "x-presence",
    title: "X / social content kit",
    blurb: "Thread drafts, bio, and a simple content calendar for X.",
    why: "Premium+ users live on X; Grok’s strength is X-aware content.",
    hosting: "Files in this project (markdown). Post manually on X — no xAI “site host.”",
    explainMode: true,
    preferAutoApprove: true,
    rules: [
      "You help with X (Twitter) presence: bio options, pinned-post ideas, thread drafts, content calendar.",
      "Use web/X search tools when available for trends; be direct and on-brand.",
      "Write files into /content or similar. Do not post for the user unless they explicitly ask and tools allow.",
      "Explain how to copy-paste publish on X.",
    ].join("\n"),
    goalTemplate:
      "Build an X content kit for: {{topic}}. Deliver: 3 bio variants, 1 pinned-post draft, 5 thread outlines, and a 2-week content calendar in markdown.",
    goalPlaceholder: "Your niche / account theme",
    loopId: null,
  },
  {
    id: "local-tool",
    title: "Local utility app",
    blurb: "Small tool that runs on your machine (CLI or simple UI).",
    why: "Practical automation without needing a public server.",
    hosting: "Runs on your computer only (npm/python). No public hosting required.",
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
    blurb: "Map and explain an existing project in plain English.",
    why: "Half of agent use is “I inherited a repo / I don’t understand it.”",
    hosting: "No deploy — learning notes in chat (and Export if you want a file).",
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
];

export function fillGoal(template: string, topic: string): string {
  const t = topic.trim() || "general / as appropriate for this folder";
  return template.replace(/\{\{topic\}\}/g, t);
}

export function getPack(id: string): StarterPack | undefined {
  return STARTER_PACKS.find((p) => p.id === id);
}

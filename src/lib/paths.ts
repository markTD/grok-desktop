/** Teaching “paths” + arsenal prompt stubs for Grok Build features. */

export type IntentPath = {
  id: string;
  title: string;
  blurb: string;
  /** Prefill for kickoff or loop */
  kind: "kickoff" | "loop" | "prompt";
  loopId?: string;
  kickoffGoal?: string;
  prompt?: string;
};

export const INTENT_PATHS: IntentPath[] = [
  {
    id: "create",
    title: "Create something",
    blurb: "Ship a feature with explore → plan → implement → verify → wrap-up.",
    kind: "loop",
    loopId: "ship-feature",
  },
  {
    id: "learn",
    title: "Learn this repo",
    blurb: "Map the codebase and explain it in plain language.",
    kind: "loop",
    loopId: "learn-codebase",
  },
  {
    id: "fix",
    title: "Fix a bug",
    blurb: "Reproduce → diagnose → minimal fix → verify.",
    kind: "loop",
    loopId: "fix-bug",
  },
  {
    id: "kickoff",
    title: "Not sure — interview me",
    blurb: "Guided kickoff writes a strong first prompt for you.",
    kind: "kickoff",
  },
];

/** One-shot arsenal prompts that lean on Grok Build capabilities. */
export const ARSENAL_PROMPTS: { id: string; label: string; hint: string; text: string }[] = [
  {
    id: "plan-mode",
    label: "Enter plan mode",
    hint: "Read-only design before edits",
    text: [
      "Enter plan mode for this task if appropriate (use enter_plan_mode).",
      "Explore the codebase, write a concrete plan to the plan file, then stop for my approval before implementing.",
      "Do not edit product code until I approve the plan.",
    ].join(" "),
  },
  {
    id: "explore-subagents",
    label: "Deep explore",
    hint: "Spawn explore subagents",
    text: [
      "Investigate this project thoroughly.",
      "Prefer spawning explore subagents for broad search so the main context stays clean.",
      "Return a structured map: entry points, key modules, risks, and how to run tests.",
      "Read-only — no file edits yet.",
    ].join(" "),
  },
  {
    id: "worktree",
    label: "Use a worktree",
    hint: "Isolate risky work",
    text: [
      "If this change is non-trivial, create or use a git worktree for isolation (prefer Grok worktree tools).",
      "Implement there, summarize the branch/worktree path, and how to merge back.",
    ].join(" "),
  },
  {
    id: "review",
    label: "Review risks",
    hint: "Critique current state",
    text: [
      "Review the current working tree / recent changes for bugs, security issues, and missing tests.",
      "Prioritize by severity with file references. Prefer read-only tools.",
    ].join(" "),
  },
  {
    id: "web-x",
    label: "Search web / X",
    hint: "Live research",
    text: [
      "Research the latest relevant information using web search and X search if available.",
      "Cite sources. Summarize what matters for this project.",
    ].join(" "),
  },
];

/**
 * Orchestration loops — multi-step prompt sequences that make Grok use
 * explore/plan/implement/verify patterns without the user engineering prompts.
 *
 * These run as sequential `session/prompt` turns in one ACP session so context
 * carries forward. Steps encourage subagents (explore/plan) for token hygiene.
 */

export type LoopStep = {
  id: string;
  /** Short label for the progress UI */
  label: string;
  /** Optional subtitle shown under the label */
  hint?: string;
  /**
   * Build the user prompt for this step.
   * `goal` is the user's plain-English objective.
   * `prior` is a compact note from earlier steps (labels only — agent has full chat).
   */
  buildPrompt: (goal: string, prior: string[]) => string;
};

export type OrchestrationLoop = {
  id: string;
  name: string;
  description: string;
  /** Who this is for */
  blurb: string;
  /** Suggested auto-approve for this loop */
  preferAutoApprove: boolean;
  /** Extra session rules when starting fresh */
  rules: string;
  steps: LoopStep[];
};

function joinPrior(prior: string[]): string {
  if (prior.length === 0) return "";
  return `\n\nCompleted earlier steps in this loop: ${prior.join(" → ")}. Build on that work; do not restart from zero.`;
}

export const ORCHESTRATION_LOOPS: OrchestrationLoop[] = [
  {
    id: "ship-feature",
    name: "Ship a feature",
    description: "Explore → plan → implement → verify",
    blurb: "Best default for building something real in an existing repo.",
    preferAutoApprove: false,
    rules: [
      "You are running a multi-step ship-feature loop in Grok Desktop.",
      "Use explore/plan subagents when investigating large areas to save main-context tokens.",
      "Prefer small reviewable diffs. Summarize files touched after implement and verify steps.",
    ].join("\n"),
    steps: [
      {
        id: "explore",
        label: "Explore",
        hint: "Map the codebase (prefer explore subagent)",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Explore (read-only).",
            `User goal: ${goal}`,
            "Investigate the codebase enough to implement this safely.",
            "Prefer spawning an explore subagent for broad search.",
            "Return: key files, current behavior, risks, and open questions.",
            "Do not edit files yet.",
            joinPrior(prior),
          ].join("\n"),
      },
      {
        id: "plan",
        label: "Plan",
        hint: "Concrete implementation plan",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Plan.",
            `User goal: ${goal}`,
            "Write a short implementation plan (bullets): files to touch, approach, tests, and risks.",
            "You may use a plan subagent. Do not implement yet unless the change is trivial.",
            joinPrior(prior),
          ].join("\n"),
      },
      {
        id: "implement",
        label: "Implement",
        hint: "Make the changes",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Implement.",
            `User goal: ${goal}`,
            "Execute the plan. Keep changes focused. Match project style.",
            "If you discover a better approach, note the deviation briefly.",
            joinPrior(prior),
          ].join("\n"),
      },
      {
        id: "verify",
        label: "Verify",
        hint: "Check work / tests",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Verify.",
            `User goal: ${goal}`,
            "Verify the change: run relevant checks/tests if available, otherwise reason carefully about failure modes.",
            "List what changed, how to test manually, and any follow-ups.",
            joinPrior(prior),
          ].join("\n"),
      },
    ],
  },
  {
    id: "fix-bug",
    name: "Fix a bug",
    description: "Reproduce → diagnose → fix → verify",
    blurb: "When something is broken and you want a careful debug path.",
    preferAutoApprove: false,
    rules: [
      "You are running a fix-bug loop in Grok Desktop.",
      "Prefer minimal fixes. Do not refactor unrelated code.",
      "Use explore subagents for wide searches.",
    ].join("\n"),
    steps: [
      {
        id: "reproduce",
        label: "Reproduce",
        hint: "Understand the failure",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Reproduce / clarify the bug.",
            `Bug report / goal: ${goal}`,
            "Find where the failure likely lives. Prefer read-only investigation.",
            "State: observed vs expected, and a minimal reproduction path.",
            joinPrior(prior),
          ].join("\n"),
      },
      {
        id: "diagnose",
        label: "Diagnose",
        hint: "Root cause",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Diagnose root cause.",
            `Bug: ${goal}`,
            "Identify the root cause with file references. Propose the smallest fix.",
            "Do not implement yet.",
            joinPrior(prior),
          ].join("\n"),
      },
      {
        id: "fix",
        label: "Fix",
        hint: "Apply minimal patch",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Fix.",
            `Bug: ${goal}`,
            "Apply the minimal fix. Avoid drive-by refactors.",
            joinPrior(prior),
          ].join("\n"),
      },
      {
        id: "verify",
        label: "Verify",
        hint: "Confirm the fix",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Verify the fix.",
            `Bug: ${goal}`,
            "Run checks/tests if possible. Confirm the failure path is addressed. Summarize.",
            joinPrior(prior),
          ].join("\n"),
      },
    ],
  },
  {
    id: "learn-codebase",
    name: "Learn this codebase",
    description: "Map → explain → hands-on",
    blurb: "Teaching mode for newcomers (and for portfolio demos).",
    preferAutoApprove: true,
    rules: [
      "You are running a learn-codebase loop in Grok Desktop.",
      "Explain in plain language. Prefer diagrams in markdown when helpful.",
      "Avoid editing product code unless the user asks; learning may include tiny demo notes.",
    ].join("\n"),
    steps: [
      {
        id: "map",
        label: "Map",
        hint: "Structure & entry points",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Map the codebase.",
            `Learning goal: ${goal || "Understand how this project works"}`,
            "Give a beginner-friendly map: folders, entry points, data flow, how to run.",
            "Prefer explore subagent for search. No code edits.",
            joinPrior(prior),
          ].join("\n"),
      },
      {
        id: "explain",
        label: "Explain",
        hint: "Deep dive on the goal",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Explain in depth.",
            `Focus: ${goal || "core architecture"}`,
            "Walk through the important code paths with file references. Use simple language.",
            joinPrior(prior),
          ].join("\n"),
      },
      {
        id: "practice",
        label: "Practice",
        hint: "Safe exercises",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Practice suggestions.",
            `Focus: ${goal || "this project"}`,
            "Propose 3 small exercises a learner could do next (easiest first).",
            "Optional: if a tiny non-destructive doc/example would help, ask before writing it.",
            joinPrior(prior),
          ].join("\n"),
      },
    ],
  },
  {
    id: "review",
    name: "Review & critique",
    description: "Scan → risks → recommendations",
    blurb: "Read-heavy review of recent work or a subsystem.",
    preferAutoApprove: true,
    rules: [
      "You are running a review loop in Grok Desktop.",
      "Default to read-only critique unless the user explicitly wants fixes.",
      "Be direct; prioritize high-severity issues.",
    ].join("\n"),
    steps: [
      {
        id: "scan",
        label: "Scan",
        hint: "Inventory what to review",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Scan.",
            `Review target: ${goal}`,
            "Identify the relevant files/diffs/modules. Summarize scope. No edits.",
            joinPrior(prior),
          ].join("\n"),
      },
      {
        id: "risks",
        label: "Risks",
        hint: "Bugs, security, design",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Risks.",
            `Review target: ${goal}`,
            "List issues by severity with file references. Include missing tests and failure modes.",
            joinPrior(prior),
          ].join("\n"),
      },
      {
        id: "recommend",
        label: "Recommend",
        hint: "What to do next",
        buildPrompt: (goal, prior) =>
          [
            "STEP: Recommendations.",
            `Review target: ${goal}`,
            "Propose a prioritized action list. Do not implement unless clearly trivial and safe.",
            joinPrior(prior),
          ].join("\n"),
      },
    ],
  },
];

export function getLoop(id: string): OrchestrationLoop | undefined {
  return ORCHESTRATION_LOOPS.find((l) => l.id === id);
}

import type { KickoffAnswers, KickoffPlan } from "./types";

/**
 * Turn a short interview into a strong first prompt + session rules.
 * This is the "prompt builder / lightweight orchestrator" — no model call.
 */
export function buildKickoffPlan(a: KickoffAnswers): KickoffPlan {
  const goal = a.goal.trim();
  const extra = a.extra.trim();

  const alwaysApprove = a.risk === "fast";

  const riskRules: Record<KickoffAnswers["risk"], string> = {
    careful:
      "Prefer read-only investigation first. Explain before destructive edits. Prefer small, reviewable diffs. Ask when unsure.",
    balanced:
      "Investigate enough to be correct, then implement. Prefer focused changes over large rewrites. Summarize what you changed.",
    fast:
      "Bias toward shipping a working solution quickly. Still avoid deleting unrelated code. Summarize at the end.",
  };

  const kindHints: Record<KickoffAnswers["projectKind"], string> = {
    existing:
      "This is an existing codebase. Inspect structure before editing. Match existing style and conventions.",
    greenfield:
      "This may be a new or empty project. Scaffold the minimum needed. Prefer simple, modern defaults.",
    learn:
      "The user wants to learn. Explain steps in plain language. Prefer teaching moments over silent magic.",
    fix: "Focus on diagnosing and fixing the reported problem. Reproduce if possible, then apply a minimal fix.",
  };

  const planBlock = a.planFirst
    ? [
        "First, produce a short plan (bullets) of what you will do.",
        "Wait for the plan to be clear in your reply before making large edits — if the task is tiny, you may implement immediately after a one-line plan.",
      ].join(" ")
    : "If the task is multi-step, keep a brief internal plan; for simple tasks just execute.";

  const rules = [
    "You are Grok Build working inside Grok Desktop (ACP client).",
    kindHints[a.projectKind],
    riskRules[a.risk],
    planBlock,
    "Prefer absolute paths under the project folder.",
    "When done, give a short checklist of what changed and how to verify.",
  ].join("\n");

  const starterParts = [
    a.planFirst
      ? `Help me with this goal. Start with a short plan, then execute.\n\nGoal:\n${goal}`
      : `Help me with this goal:\n\n${goal}`,
  ];

  if (extra) {
    starterParts.push(`\n\nContext / constraints:\n${extra}`);
  }

  starterParts.push(
    "\n\nWork in this project folder. Use tools as needed. Keep changes focused.",
  );

  const summary = [
    a.projectKind,
    a.risk,
    a.planFirst ? "plan-first" : "direct",
    alwaysApprove ? "auto-approve" : "ask-on-tools",
  ].join(" · ");

  return {
    starterPrompt: starterParts.join(""),
    rules,
    alwaysApprove,
    summary,
  };
}

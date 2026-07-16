/** Explain mode — teach while working (session rules + per-turn nudge). */

export const EXPLAIN_MODE_RULES = [
  "EXPLAIN MODE is ON for this session.",
  "Teach as you work: use plain language a non-expert can follow.",
  "Before non-trivial edits, say in 1–3 short bullets what you will change and why.",
  "After edits, summarize what changed and how the user can verify.",
  "Define jargon once when you first use it.",
  "Prefer small steps over silent magic.",
].join("\n");

/** Appended to each user prompt so mid-session toggle still works. */
export function withExplainNudge(userText: string, explainMode: boolean): string {
  if (!explainMode) return userText;
  return [
    userText.trim(),
    "",
    "—",
    "(Explain mode: teach in plain language as you work; brief why before big edits; short verify steps after.)",
  ].join("\n");
}

export function mergeSessionRules(
  base: string | null | undefined,
  explainMode: boolean,
): string | null {
  const parts: string[] = [];
  if (base?.trim()) parts.push(base.trim());
  if (explainMode) parts.push(EXPLAIN_MODE_RULES);
  if (parts.length === 0) return null;
  return parts.join("\n\n");
}

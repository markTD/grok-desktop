export type UsageSnapshot = {
  inputTokens?: number;
  outputTokens?: number;
  totalTokens?: number;
  cachedReadTokens?: number;
  reasoningTokens?: number;
  modelId?: string;
  turns: number;
};

export function emptyUsage(): UsageSnapshot {
  return { turns: 0 };
}

/** Extract usage fields from ACP prompt `_meta` (best-effort). */
export function usageFromMeta(meta: Record<string, unknown> | null | undefined): Partial<UsageSnapshot> {
  if (!meta) return {};
  const usage =
    meta.usage && typeof meta.usage === "object"
      ? (meta.usage as Record<string, unknown>)
      : meta;

  const num = (v: unknown) => (typeof v === "number" ? v : undefined);

  return {
    inputTokens: num(usage.inputTokens) ?? num(usage.input_tokens),
    outputTokens: num(usage.outputTokens) ?? num(usage.output_tokens),
    totalTokens: num(usage.totalTokens) ?? num(usage.total_tokens),
    cachedReadTokens: num(usage.cachedReadTokens) ?? num(usage.cached_read_tokens),
    reasoningTokens: num(usage.reasoningTokens) ?? num(usage.reasoning_tokens),
    modelId: typeof meta.modelId === "string" ? meta.modelId : undefined,
  };
}

export function accumulateUsage(
  prev: UsageSnapshot,
  meta: Record<string, unknown> | null | undefined,
): UsageSnapshot {
  const u = usageFromMeta(meta);
  const add = (a?: number, b?: number) =>
    a !== undefined || b !== undefined ? (a ?? 0) + (b ?? 0) : undefined;

  return {
    inputTokens: add(prev.inputTokens, u.inputTokens),
    outputTokens: add(prev.outputTokens, u.outputTokens),
    totalTokens: add(prev.totalTokens, u.totalTokens),
    cachedReadTokens: add(prev.cachedReadTokens, u.cachedReadTokens),
    reasoningTokens: add(prev.reasoningTokens, u.reasoningTokens),
    modelId: u.modelId ?? prev.modelId,
    turns: prev.turns + 1,
  };
}

export function formatUsage(u: UsageSnapshot): string {
  const parts: string[] = [`${u.turns} turn${u.turns === 1 ? "" : "s"}`];
  if (u.totalTokens !== undefined) parts.push(`${u.totalTokens.toLocaleString()} tokens`);
  else if (u.inputTokens !== undefined || u.outputTokens !== undefined) {
    parts.push(
      `${(u.inputTokens ?? 0).toLocaleString()} in / ${(u.outputTokens ?? 0).toLocaleString()} out`,
    );
  }
  if (u.cachedReadTokens) parts.push(`${u.cachedReadTokens.toLocaleString()} cached`);
  return parts.join(" · ");
}

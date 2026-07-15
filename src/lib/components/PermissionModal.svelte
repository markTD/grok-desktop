<script lang="ts">
  import type { PermissionEvent, PermissionOption } from "$lib/types";

  let {
    request,
    onReply,
  }: {
    request: PermissionEvent | null;
    onReply: (optionId: string | null, cancelled?: boolean) => void;
  } = $props();

  function optionId(o: PermissionOption): string {
    return String(o.optionId ?? o.id ?? "");
  }

  function optionLabel(o: PermissionOption): string {
    return String(o.name ?? o.label ?? o.kind ?? optionId(o) ?? "Option");
  }

  function toolTitle(req: PermissionEvent): string {
    const tc = req.toolCall;
    if (!tc) return "Tool permission";
    if (typeof tc.title === "string") return tc.title;
    if (typeof tc.toolName === "string") return tc.toolName;
    return "Tool permission";
  }
</script>

{#if request}
  <div class="overlay" role="dialog" aria-modal="true" aria-labelledby="perm-title">
    <div class="card">
      <h2 id="perm-title">Allow this tool?</h2>
      <p class="tool">{toolTitle(request)}</p>
      {#if request.toolCall && typeof request.toolCall.kind === "string"}
        <p class="muted">{request.toolCall.kind}</p>
      {/if}
      <div class="actions">
        {#each request.options as opt}
          <button type="button" class="btn primary" onclick={() => onReply(optionId(opt))}>
            {optionLabel(opt)}
          </button>
        {/each}
        {#if request.options.length === 0}
          <button type="button" class="btn primary" onclick={() => onReply("allow-once")}>
            Allow once
          </button>
        {/if}
        <button type="button" class="btn danger" onclick={() => onReply(null, true)}>Deny</button>
      </div>
      <p class="hint">
        Grok Build is asking your OK before a tool runs. Choose carefully on production folders.
      </p>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 90;
    background: rgba(0, 0, 0, 0.55);
    display: grid;
    place-items: center;
    padding: 1rem;
  }

  .card {
    width: min(420px, 100%);
    background: #151922;
    border: 1px solid #2a3344;
    border-radius: 14px;
    padding: 1.15rem 1.25rem;
  }

  h2 {
    margin: 0 0 0.5rem;
    font-size: 1.1rem;
  }

  .tool {
    margin: 0;
    font-family: ui-monospace, Menlo, monospace;
    color: #6ee7b7;
    font-size: 0.9rem;
  }

  .muted {
    margin: 0.25rem 0 0;
    color: #8b93a7;
    font-size: 0.8rem;
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.45rem;
    margin-top: 1rem;
  }

  .btn {
    border: 1px solid #2a3344;
    background: #1a2030;
    color: #e8eaed;
    border-radius: 8px;
    padding: 0.45rem 0.8rem;
    font-size: 0.85rem;
    cursor: pointer;
  }

  .btn.primary {
    background: #1d4ed8;
    border-color: #2563eb;
  }

  .btn.danger {
    background: #3f1d1d;
    border-color: #7f1d1d;
  }

  .hint {
    margin: 0.85rem 0 0;
    font-size: 0.75rem;
    color: #8b93a7;
    line-height: 1.4;
  }
</style>

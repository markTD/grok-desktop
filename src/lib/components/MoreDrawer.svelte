<script lang="ts">
  import type { ModelInfo } from "$lib/build";
  import { ARSENAL_PROMPTS } from "$lib/paths";
  import HelpTip from "$lib/components/HelpTip.svelte";

  let {
    open = false,
    connected = false,
    alwaysApprove = $bindable(false),
    effortChoice = $bindable("high"),
    modelChoice = $bindable(""),
    models = [] as ModelInfo[],
    isGit = true,
    onClose,
    onExport,
    onSetup,
    onBuild,
    onOpenSession,
    onOpenPlan,
    onArsenal,
    canExport = false,
    canSession = false,
    exporting = false,
  }: {
    open?: boolean;
    connected?: boolean;
    alwaysApprove?: boolean;
    effortChoice?: string;
    modelChoice?: string;
    models?: ModelInfo[];
    isGit?: boolean;
    onClose: () => void;
    onExport: () => void;
    onSetup: () => void;
    onBuild: () => void;
    onOpenSession: () => void;
    onOpenPlan: () => void;
    onArsenal: (text: string) => void;
    canExport?: boolean;
    canSession?: boolean;
    exporting?: boolean;
  } = $props();
</script>

{#if open}
  <div class="overlay" role="dialog" aria-modal="true" aria-labelledby="more-title">
    <div class="card">
      <header>
        <h2 id="more-title">More · settings & arsenal</h2>
        <button type="button" class="x" onclick={onClose} aria-label="Close">×</button>
      </header>

      <section>
        <h3>Session controls</h3>
        <p class="muted small">Applied on next <strong>Connect</strong> (not mid-session).</p>
        <label class="row">
          <span>Effort</span>
          <select bind:value={effortChoice} disabled={connected}>
            <option value="low">low</option>
            <option value="medium">medium</option>
            <option value="high">high</option>
            <option value="xhigh">xhigh</option>
          </select>
        </label>
        <label class="row">
          <span>Model</span>
          {#if models.length}
            <select bind:value={modelChoice} disabled={connected}>
              <option value="">default</option>
              {#each models as m}
                <option value={m.id}>{m.name}</option>
              {/each}
            </select>
          {:else}
            <input bind:value={modelChoice} disabled={connected} placeholder="default" />
          {/if}
        </label>
        <label class="check">
          <input type="checkbox" bind:checked={alwaysApprove} disabled={connected} />
          Auto-approve tools
          <HelpTip title="Auto-approve" label="?">
            <p>Faster vibe coding. Prefer git repos so you can revert. Off by default for safety.</p>
          </HelpTip>
        </label>
        {#if !isGit && alwaysApprove}
          <p class="warn">⚠ Folder is not a git repo — auto-approve is riskier.</p>
        {/if}
      </section>

      <section>
        <h3>Build arsenal (prompts)</h3>
        <p class="muted small">Sends a prompt that steers Grok Build features (plan mode, subagents, worktrees, search).</p>
        <div class="arsenal">
          {#each ARSENAL_PROMPTS as a}
            <button
              type="button"
              class="chip"
              disabled={!connected}
              title={a.hint}
              onclick={() => onArsenal(a.text)}
            >
              {a.label}
            </button>
          {/each}
        </div>
      </section>

      <section>
        <h3>Project & session</h3>
        <div class="actions">
          <button type="button" class="btn" onclick={onExport} disabled={!canExport || exporting}>
            {exporting ? "Exporting…" : "Export notes"}
          </button>
          <button type="button" class="btn" onclick={onOpenSession} disabled={!canSession}>
            Open session folder
          </button>
          <button type="button" class="btn" onclick={onOpenPlan} disabled={!canSession}>
            Open plan file
          </button>
        </div>
      </section>

      <section>
        <h3>App</h3>
        <div class="actions">
          <button type="button" class="btn" onclick={onBuild}>Build monitor</button>
          <button type="button" class="btn" onclick={onSetup}>Guided setup</button>
        </div>
      </section>

      <footer>
        <button type="button" class="btn primary" onclick={onClose}>Close</button>
      </footer>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 72;
    background: rgba(0, 0, 0, 0.5);
    display: grid;
    place-items: center;
    padding: 1rem;
  }

  .card {
    width: min(440px, 100%);
    max-height: min(90vh, 720px);
    overflow: auto;
    background: #151922;
    border: 1px solid #2a3344;
    border-radius: 14px;
    padding: 1rem 1.15rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  h2 {
    margin: 0;
    font-size: 1.05rem;
  }

  h3 {
    margin: 0 0 0.35rem;
    font-size: 0.82rem;
    color: #a5b4fc;
  }

  section {
    margin-top: 0.95rem;
  }

  .x {
    border: none;
    background: transparent;
    color: #8b93a7;
    font-size: 1.35rem;
    cursor: pointer;
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    margin: 0.35rem 0;
    font-size: 0.85rem;
    color: #c5cad6;
  }

  .row select,
  .row input {
    background: #0d0f12;
    border: 1px solid #2a3344;
    border-radius: 6px;
    color: #e8eaed;
    padding: 0.3rem 0.45rem;
    font-size: 0.8rem;
    min-width: 9rem;
  }

  .check {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.85rem;
    color: #c5cad6;
    margin-top: 0.45rem;
  }

  .warn {
    margin: 0.4rem 0 0;
    font-size: 0.8rem;
    color: #fde68a;
  }

  .arsenal {
    display: flex;
    flex-wrap: wrap;
    gap: 0.35rem;
  }

  .chip {
    border: 1px solid #2a3344;
    background: #0d0f12;
    color: #e8eaed;
    border-radius: 999px;
    padding: 0.35rem 0.65rem;
    font-size: 0.78rem;
    cursor: pointer;
  }

  .chip:hover:not(:disabled) {
    border-color: #3b82f6;
  }

  .chip:disabled {
    opacity: 0.45;
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
  }

  .btn {
    border: 1px solid #2a3344;
    background: #1a2030;
    color: #e8eaed;
    border-radius: 8px;
    padding: 0.4rem 0.7rem;
    font-size: 0.82rem;
    cursor: pointer;
  }

  .btn.primary {
    background: #1d4ed8;
    border-color: #2563eb;
  }

  .btn:disabled {
    opacity: 0.45;
  }

  footer {
    display: flex;
    justify-content: flex-end;
    margin-top: 1rem;
  }

  .muted {
    color: #8b93a7;
  }
  .small {
    font-size: 0.78rem;
    margin: 0 0 0.4rem;
  }
</style>

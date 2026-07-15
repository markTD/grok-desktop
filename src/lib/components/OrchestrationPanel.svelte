<script lang="ts">
  import { ORCHESTRATION_LOOPS, type OrchestrationLoop } from "$lib/loops";
  import type { LoopPhase } from "$lib/types";
  import HelpTip from "$lib/components/HelpTip.svelte";

  let {
    open = false,
    connected = false,
    busy = false,
    loopRunning = false,
    loopPhase = "idle" as LoopPhase,
    currentStepIndex = -1,
    activeLoopId = null as string | null,
    lastLoopName = null as string | null,
    lastLoopGoal = null as string | null,
    onClose,
    onStart,
    onStop,
    onDismissResult,
  }: {
    open?: boolean;
    connected?: boolean;
    busy?: boolean;
    loopRunning?: boolean;
    loopPhase?: LoopPhase;
    currentStepIndex?: number;
    activeLoopId?: string | null;
    lastLoopName?: string | null;
    lastLoopGoal?: string | null;
    onClose: () => void;
    onStart: (loop: OrchestrationLoop, goal: string) => void;
    onStop: () => void;
    onDismissResult: () => void;
  } = $props();

  let selectedId = $state(ORCHESTRATION_LOOPS[0]?.id ?? "ship-feature");
  let goal = $state("");

  let selected = $derived(
    ORCHESTRATION_LOOPS.find((l) => l.id === selectedId) ?? ORCHESTRATION_LOOPS[0],
  );

  let activeLoop = $derived(
    ORCHESTRATION_LOOPS.find((l) => l.id === activeLoopId) ?? null,
  );

  let showProgress = $derived(
    loopPhase === "running" || loopPhase === "complete" || loopPhase === "stopped",
  );
</script>

{#if open}
  <div class="overlay" role="dialog" aria-modal="true" aria-labelledby="orch-title">
    <div class="card">
      <header>
        <div>
          <p class="eyebrow">Orchestration</p>
          <h2 id="orch-title">Multi-step loops</h2>
        </div>
        <div class="head-actions">
          <HelpTip title="Why loops?" label="?">
            <p>
              Short sequence of turns (explore → plan → implement → verify) instead of one giant
              prompt. Keeps work focused and encourages Grok to use explore/plan subagents.
            </p>
          </HelpTip>
          <button
            type="button"
            class="x"
            onclick={onClose}
            aria-label="Close"
            disabled={loopPhase === "running"}
          >
            ×
          </button>
        </div>
      </header>

      {#if showProgress && (activeLoop || lastLoopName)}
        <div class="running" class:done={loopPhase === "complete"} class:stopped={loopPhase === "stopped"}>
          {#if loopPhase === "complete"}
            <div class="banner ok" role="status">
              <strong>✓ Loop finished</strong>
              <span
                >{lastLoopName ?? activeLoop?.name}
                {#if lastLoopGoal}
                  — {lastLoopGoal}
                {/if}
              </span>
              <p>
                All steps completed. Scroll the chat for results. You can keep chatting in this
                session or run another loop.
              </p>
            </div>
          {:else if loopPhase === "stopped"}
            <div class="banner stop" role="status">
              <strong>Loop stopped</strong>
              <span>You can resume by starting a new loop or chatting freely.</span>
            </div>
          {:else}
            <p class="now">
              Running <strong>{activeLoop?.name}</strong>
              {#if lastLoopGoal}<span class="goal">· {lastLoopGoal}</span>{/if}
            </p>
          {/if}

          <ol class="steps">
            {#each (activeLoop?.steps ?? ORCHESTRATION_LOOPS.find((l) => l.name === lastLoopName)?.steps ?? []) as step, i}
              <li
                class:done={loopPhase === "complete" || i < currentStepIndex || (loopPhase !== "running" && i < currentStepIndex)}
                class:active={loopPhase === "running" && i === currentStepIndex}
                class:alldone={loopPhase === "complete"}
              >
                <span class="idx">{loopPhase === "complete" || i < currentStepIndex ? "✓" : i + 1}</span>
                {step.label}
                {#if step.hint}<span class="hint">· {step.hint}</span>{/if}
              </li>
            {/each}
          </ol>

          {#if loopPhase === "running"}
            <button type="button" class="btn danger" onclick={onStop}>Stop loop</button>
          {:else}
            <div class="footer-row">
              <button type="button" class="btn ghost" onclick={onDismissResult}>Close</button>
              <button
                type="button"
                class="btn primary"
                onclick={onDismissResult}
              >
                Done — back to chat
              </button>
            </div>
          {/if}
        </div>
      {:else}
        <div class="loops">
          {#each ORCHESTRATION_LOOPS as loop}
            <button
              type="button"
              class="loop"
              class:on={selectedId === loop.id}
              onclick={() => (selectedId = loop.id)}
            >
              <strong>{loop.name}</strong>
              <span>{loop.description}</span>
              <em>{loop.blurb}</em>
            </button>
          {/each}
        </div>

        {#if selected}
          <label for="loop-goal">What should this loop achieve?</label>
          <textarea
            id="loop-goal"
            rows="3"
            bind:value={goal}
            placeholder="e.g. Add export-to-CSV on the reports page"
            disabled={busy}
          ></textarea>
          <p class="muted">
            {selected.steps.length} steps
            {#if selected.preferAutoApprove}
              · suggests auto-approve
            {:else}
              · careful by default
            {/if}
            {#if !connected}
              · will connect first if needed
            {/if}
          </p>
        {/if}

        <footer>
          <button type="button" class="btn ghost" onclick={onClose}>Cancel</button>
          <button
            type="button"
            class="btn primary"
            disabled={!goal.trim() || busy || !selected}
            onclick={() => selected && onStart(selected, goal.trim())}
          >
            Run loop
          </button>
        </footer>
      {/if}
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 70;
    background: rgba(0, 0, 0, 0.55);
    display: grid;
    place-items: center;
    padding: 1rem;
  }

  .card {
    width: min(560px, 100%);
    max-height: min(90vh, 760px);
    overflow: auto;
    background: #151922;
    border: 1px solid #2a3344;
    border-radius: 14px;
    padding: 1.1rem 1.25rem 1rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    gap: 0.75rem;
    align-items: flex-start;
  }

  .head-actions {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }

  .eyebrow {
    margin: 0;
    font-size: 0.72rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: #7dd3fc;
  }

  h2 {
    margin: 0.2rem 0 0;
    font-size: 1.1rem;
  }

  .x {
    border: none;
    background: transparent;
    color: #8b93a7;
    font-size: 1.4rem;
    cursor: pointer;
  }

  .x:disabled {
    opacity: 0.35;
    cursor: default;
  }

  .loops {
    display: grid;
    gap: 0.4rem;
    margin: 1rem 0 0.85rem;
  }

  .loop {
    text-align: left;
    border: 1px solid #2a3344;
    background: #0d0f12;
    color: #e8eaed;
    border-radius: 10px;
    padding: 0.65rem 0.75rem;
    cursor: pointer;
    display: grid;
    gap: 0.15rem;
  }

  .loop.on {
    border-color: #3b82f6;
    background: #1a2744;
  }

  .loop span {
    font-size: 0.8rem;
    color: #a5b4fc;
  }

  .loop em {
    font-style: normal;
    font-size: 0.78rem;
    color: #8b93a7;
  }

  label {
    display: block;
    font-size: 0.85rem;
    color: #c5cad6;
    margin-bottom: 0.35rem;
  }

  textarea {
    width: 100%;
    box-sizing: border-box;
    border-radius: 10px;
    border: 1px solid #2a3344;
    background: #0d0f12;
    color: #e8eaed;
    padding: 0.65rem 0.75rem;
    font: inherit;
    font-size: 0.9rem;
    resize: vertical;
  }

  .muted {
    margin: 0.4rem 0 0;
    font-size: 0.78rem;
    color: #8b93a7;
  }

  footer,
  .footer-row {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    margin-top: 1rem;
  }

  .btn {
    border: 1px solid #2a3344;
    background: #1a2030;
    color: #e8eaed;
    border-radius: 8px;
    padding: 0.45rem 0.9rem;
    font-size: 0.85rem;
    cursor: pointer;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: default;
  }

  .btn.primary {
    background: #1d4ed8;
    border-color: #2563eb;
  }

  .btn.danger {
    background: #3f1d1d;
    border-color: #7f1d1d;
  }

  .btn.ghost {
    background: transparent;
  }

  .now {
    margin: 0.75rem 0 0.5rem;
  }

  .goal {
    color: #8b93a7;
    font-size: 0.9em;
  }

  .banner {
    margin: 0.75rem 0 0.65rem;
    padding: 0.75rem 0.85rem;
    border-radius: 10px;
    display: grid;
    gap: 0.25rem;
  }

  .banner.ok {
    background: #0f2a1f;
    border: 1px solid #166534;
    color: #bbf7d0;
  }

  .banner.stop {
    background: #2a2110;
    border: 1px solid #854d0e;
    color: #fde68a;
  }

  .banner p {
    margin: 0.35rem 0 0;
    font-size: 0.82rem;
    opacity: 0.95;
    line-height: 1.4;
  }

  .banner span {
    font-size: 0.85rem;
    opacity: 0.9;
  }

  .steps {
    margin: 0 0 1rem;
    padding: 0;
    list-style: none;
    display: grid;
    gap: 0.35rem;
  }

  .steps li {
    display: flex;
    align-items: center;
    gap: 0.45rem;
    font-size: 0.88rem;
    color: #8b93a7;
    padding: 0.35rem 0.5rem;
    border-radius: 8px;
  }

  .steps li.done,
  .steps li.alldone {
    color: #34d399;
  }

  .steps li.active {
    color: #e8eaed;
    background: #1a2744;
  }

  .idx {
    width: 1.35rem;
    height: 1.35rem;
    border-radius: 999px;
    border: 1px solid #2a3344;
    display: inline-grid;
    place-items: center;
    font-size: 0.72rem;
    flex-shrink: 0;
  }

  .hint {
    color: #8b93a7;
    font-size: 0.78rem;
  }
</style>

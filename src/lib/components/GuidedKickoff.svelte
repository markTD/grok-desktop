<script lang="ts">
  import { buildKickoffPlan } from "$lib/kickoff";
  import type { KickoffAnswers, KickoffPlan } from "$lib/types";

  let {
    open = false,
    onClose,
    onLaunch,
  }: {
    open?: boolean;
    onClose: () => void;
    onLaunch: (plan: KickoffPlan, answers: KickoffAnswers) => void;
  } = $props();

  let step = $state(0);
  let goal = $state("");
  let projectKind = $state<KickoffAnswers["projectKind"]>("existing");
  let risk = $state<KickoffAnswers["risk"]>("balanced");
  let planFirst = $state(true);
  let extra = $state("");

  let preview = $derived(
    buildKickoffPlan({
      goal: goal || "(your goal)",
      projectKind,
      risk,
      planFirst,
      extra,
    }),
  );

  function answers(): KickoffAnswers {
    return { goal, projectKind, risk, planFirst, extra };
  }

  function next() {
    if (step === 0 && !goal.trim()) return;
    if (step < 3) step += 1;
    else onLaunch(preview, answers());
  }

  function back() {
    if (step > 0) step -= 1;
    else onClose();
  }
</script>

{#if open}
  <div class="overlay" role="dialog" aria-modal="true" aria-labelledby="gk-title">
    <div class="card">
      <header>
        <div>
          <p class="eyebrow">Guided kickoff</p>
          <h2 id="gk-title">Tell us what you want — we set up Grok</h2>
        </div>
        <button type="button" class="x" onclick={onClose} aria-label="Close">×</button>
      </header>

      <div class="progress" aria-hidden="true">
        {#each [0, 1, 2, 3] as i}
          <span class:on={i <= step}></span>
        {/each}
      </div>

      {#if step === 0}
        <label for="goal">What do you want to build or fix?</label>
        <textarea
          id="goal"
          rows="4"
          bind:value={goal}
          placeholder="e.g. Add a dark mode toggle to the settings page and make sure it saves."
        ></textarea>
        <p class="hint">Plain English is perfect. No special prompt skills needed.</p>
      {:else if step === 1}
        <p class="label">What kind of work is this?</p>
        <div class="choices">
          <button type="button" class:on={projectKind === "existing"} onclick={() => (projectKind = "existing")}>
            Existing app — match the codebase
          </button>
          <button type="button" class:on={projectKind === "greenfield"} onclick={() => (projectKind = "greenfield")}>
            New idea — scaffold from scratch
          </button>
          <button type="button" class:on={projectKind === "fix"} onclick={() => (projectKind = "fix")}>
            Bug / broken thing — diagnose & fix
          </button>
          <button type="button" class:on={projectKind === "learn"} onclick={() => (projectKind = "learn")}>
            Learning — explain as we go
          </button>
        </div>
      {:else if step === 2}
        <p class="label">How bold should Grok be?</p>
        <div class="choices">
          <button type="button" class:on={risk === "careful"} onclick={() => (risk = "careful")}>
            Careful — read first, small diffs
          </button>
          <button type="button" class:on={risk === "balanced"} onclick={() => (risk = "balanced")}>
            Balanced — solid default
          </button>
          <button type="button" class:on={risk === "fast"} onclick={() => (risk = "fast")}>
            Fast vibe — auto-approve tools
          </button>
        </div>
        <label class="check">
          <input type="checkbox" bind:checked={planFirst} />
          Plan first (recommended for bigger goals)
        </label>
      {:else}
        <label for="extra">Anything else? (optional)</label>
        <textarea
          id="extra"
          rows="3"
          bind:value={extra}
          placeholder="Stack preferences, files to avoid, deadline, etc."
        ></textarea>
        <div class="preview">
          <p class="label">We’ll start with</p>
          <p class="summary">{preview.summary}</p>
          <pre class="prompt">{preview.starterPrompt}</pre>
        </div>
      {/if}

      <footer>
        <button type="button" class="btn ghost" onclick={back}>
          {step === 0 ? "Cancel" : "Back"}
        </button>
        <button
          type="button"
          class="btn primary"
          onclick={next}
          disabled={step === 0 && !goal.trim()}
        >
          {step === 3 ? "Connect & launch" : "Continue"}
        </button>
      </footer>
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
    width: min(520px, 100%);
    max-height: min(90vh, 720px);
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
    line-height: 1;
  }

  .progress {
    display: flex;
    gap: 0.35rem;
    margin: 0.9rem 0 1rem;
  }

  .progress span {
    flex: 1;
    height: 3px;
    border-radius: 2px;
    background: #2a3344;
  }

  .progress span.on {
    background: #3b82f6;
  }

  label,
  .label {
    display: block;
    font-size: 0.85rem;
    color: #c5cad6;
    margin-bottom: 0.4rem;
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

  .hint {
    margin: 0.4rem 0 0;
    font-size: 0.78rem;
    color: #8b93a7;
  }

  .choices {
    display: grid;
    gap: 0.4rem;
  }

  .choices button {
    text-align: left;
    border: 1px solid #2a3344;
    background: #0d0f12;
    color: #e8eaed;
    border-radius: 10px;
    padding: 0.65rem 0.75rem;
    font-size: 0.88rem;
    cursor: pointer;
  }

  .choices button.on {
    border-color: #3b82f6;
    background: #1a2744;
  }

  .check {
    display: flex;
    align-items: center;
    gap: 0.45rem;
    margin-top: 0.85rem;
    font-size: 0.85rem;
    color: #c5cad6;
  }

  .preview {
    margin-top: 0.5rem;
  }

  .summary {
    margin: 0 0 0.4rem;
    font-size: 0.8rem;
    color: #a5b4fc;
  }

  .prompt {
    margin: 0;
    max-height: 160px;
    overflow: auto;
    white-space: pre-wrap;
    font-size: 0.78rem;
    background: #0d0f12;
    border: 1px solid #232a38;
    border-radius: 8px;
    padding: 0.55rem 0.65rem;
    color: #c5cad6;
  }

  footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    margin-top: 1.1rem;
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

  .btn.ghost {
    background: transparent;
  }
</style>

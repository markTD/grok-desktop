<script lang="ts">
  import { STARTER_PACKS, type StarterPack } from "$lib/packs";
  import HelpTip from "$lib/components/HelpTip.svelte";

  let {
    open = false,
    busy = false,
    onClose,
    onLaunch,
  }: {
    open?: boolean;
    busy?: boolean;
    onClose: () => void;
    onLaunch: (pack: StarterPack, goal: string) => void;
  } = $props();

  let selectedId = $state(STARTER_PACKS[0]?.id ?? "personal-site");
  let topic = $state("");

  let selected = $derived(
    STARTER_PACKS.find((p) => p.id === selectedId) ?? STARTER_PACKS[0],
  );

  function previewGoal(pack: StarterPack): string {
    const t = topic.trim() || "…";
    return pack.goalTemplate.replace(/\{\{topic\}\}/g, t);
  }
</script>

{#if open}
  <div class="overlay" role="dialog" aria-modal="true" aria-labelledby="pack-title">
    <div class="card">
      <header>
        <div>
          <p class="eyebrow">Starter packs</p>
          <h2 id="pack-title">What do you want to make?</h2>
        </div>
        <div class="head">
          <HelpTip title="Hosting" label="?">
            <p>
              <strong>xAI does not host public websites.</strong> Grok builds code on your machine.
              Public URLs usually mean GitHub Pages, Netlify, or Vercel — or just run locally.
            </p>
          </HelpTip>
          <button type="button" class="x" onclick={onClose} aria-label="Close">×</button>
        </div>
      </header>

      <p class="lead">
        Popular first projects for SuperGrok / Premium+ users. Pick one, add a short detail, launch.
      </p>

      <div class="packs">
        {#each STARTER_PACKS as pack}
          <button
            type="button"
            class="pack"
            class:on={selectedId === pack.id}
            onclick={() => (selectedId = pack.id)}
          >
            <strong>{pack.title}</strong>
            <span>{pack.blurb}</span>
          </button>
        {/each}
      </div>

      {#if selected}
        <div class="detail">
          <p class="why"><strong>Why people pick this:</strong> {selected.why}</p>
          <p class="host"><strong>Where it lives:</strong> {selected.hosting}</p>
          <label for="topic">Your detail (optional)</label>
          <input
            id="topic"
            bind:value={topic}
            placeholder={selected.goalPlaceholder}
            disabled={busy}
          />
          <p class="muted small">We’ll send something like:</p>
          <pre class="preview">{previewGoal(selected)}</pre>
        </div>
      {/if}

      <footer>
        <button type="button" class="btn ghost" onclick={onClose}>Cancel</button>
        <button
          type="button"
          class="btn primary"
          disabled={busy || !selected}
          onclick={() => selected && onLaunch(selected, topic)}
        >
          Launch pack
        </button>
      </footer>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 74;
    background: rgba(0, 0, 0, 0.55);
    display: grid;
    place-items: center;
    padding: 1rem;
  }

  .card {
    width: min(540px, 100%);
    max-height: min(92vh, 800px);
    overflow: auto;
    background: #151922;
    border: 1px solid #2a3344;
    border-radius: 14px;
    padding: 1.1rem 1.2rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .head {
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

  .lead {
    margin: 0.65rem 0 0.75rem;
    font-size: 0.88rem;
    color: #8b93a7;
    line-height: 1.45;
  }

  .x {
    border: none;
    background: transparent;
    color: #8b93a7;
    font-size: 1.4rem;
    cursor: pointer;
  }

  .packs {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.4rem;
  }

  @media (max-width: 520px) {
    .packs {
      grid-template-columns: 1fr;
    }
  }

  .pack {
    text-align: left;
    border: 1px solid #2a3344;
    background: #0d0f12;
    color: #e8eaed;
    border-radius: 10px;
    padding: 0.55rem 0.65rem;
    cursor: pointer;
    display: grid;
    gap: 0.2rem;
  }

  .pack.on {
    border-color: #3b82f6;
    background: #1a2744;
  }

  .pack span {
    font-size: 0.75rem;
    color: #8b93a7;
    line-height: 1.35;
  }

  .detail {
    margin-top: 0.85rem;
  }

  .why,
  .host {
    margin: 0 0 0.45rem;
    font-size: 0.82rem;
    color: #c5cad6;
    line-height: 1.4;
  }

  label {
    display: block;
    font-size: 0.8rem;
    color: #8b93a7;
    margin: 0.5rem 0 0.3rem;
  }

  input {
    width: 100%;
    box-sizing: border-box;
    border-radius: 8px;
    border: 1px solid #2a3344;
    background: #0d0f12;
    color: #e8eaed;
    padding: 0.5rem 0.65rem;
    font: inherit;
    font-size: 0.88rem;
  }

  .preview {
    margin: 0.35rem 0 0;
    padding: 0.55rem 0.65rem;
    background: #0d0f12;
    border: 1px solid #232a38;
    border-radius: 8px;
    font-size: 0.75rem;
    color: #a5b4fc;
    white-space: pre-wrap;
    max-height: 100px;
    overflow: auto;
  }

  footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.45rem;
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

  .btn.primary {
    background: #1d4ed8;
    border-color: #2563eb;
  }

  .btn.ghost {
    background: transparent;
  }

  .btn:disabled {
    opacity: 0.5;
  }

  .muted {
    color: #8b93a7;
  }

  .small {
    font-size: 0.75rem;
    margin: 0.4rem 0 0;
  }
</style>

<script lang="ts">
  import type { GrokStatus } from "$lib/types";
  import { openUrl } from "@tauri-apps/plugin-opener";

  let {
    status,
    statusLoading = false,
    cwd,
    onRecheck,
    onPickFolder,
    onDone,
    onKickoff,
    onLoops,
    onConnect,
  }: {
    status: GrokStatus | null;
    statusLoading?: boolean;
    cwd: string;
    onRecheck: () => void;
    onPickFolder: () => void;
    onDone: () => void;
    onKickoff: () => void;
    onLoops: () => void;
    onConnect: () => void;
  } = $props();

  let step = $state(0);

  const installCmd =
    "curl -fsSL https://x.ai/cli/install.sh | bash";

  let cliOk = $derived(!!status?.ready);
  let hasFolder = $derived(cwd.trim().length > 1);

  async function openInstallDocs() {
    try {
      await openUrl("https://x.ai/cli");
    } catch {
      /* ignore */
    }
  }

  async function copyInstall() {
    try {
      await navigator.clipboard.writeText(installCmd);
    } catch {
      /* ignore */
    }
  }

  function next() {
    if (step < 4) step += 1;
    else onDone();
  }

  function back() {
    if (step > 0) step -= 1;
  }
</script>

<div class="overlay" role="dialog" aria-modal="true" aria-labelledby="setup-title">
  <div class="card">
    <p class="step-label">Guided setup · {step + 1} / 5</p>

    {#if step === 0}
      <h2 id="setup-title">Welcome to Grok Desktop</h2>
      <p class="body">
        A friendly window on top of <strong>xAI Grok Build</strong> — the coding agent for SuperGrok
        / X Premium+ subscribers. You create or learn in a project folder; Grok does the heavy tool
        work in the official CLI.
      </p>
      <p class="muted">Takes about two minutes if the CLI is already installed.</p>
    {:else if step === 1}
      <h2 id="setup-title">Install Grok Build CLI</h2>
      <p class="body">In Terminal (macOS / Linux):</p>
      <pre class="code">{installCmd}</pre>
      <div class="row">
        <button type="button" class="btn" onclick={copyInstall}>Copy command</button>
        <button type="button" class="btn" onclick={openInstallDocs}>Open x.ai/cli</button>
        <button type="button" class="btn" onclick={onRecheck} disabled={statusLoading}>
          {statusLoading ? "Checking…" : "I installed it — recheck"}
        </button>
      </div>
      <p class="status" class:ok={cliOk} class:bad={status && !cliOk}>
        {#if statusLoading}
          Checking…
        {:else if cliOk}
          ✓ CLI ready — {status?.version ?? "found"}
        {:else}
          Not detected yet. Install, then recheck. ({status?.message ?? "waiting"})
        {/if}
      </p>
    {:else if step === 2}
      <h2 id="setup-title">Sign in</h2>
      <p class="body">
        Run <code>grok</code> once in a terminal and complete browser login (same account as X /
        SuperGrok). Then recheck here.
      </p>
      <div class="row">
        <button type="button" class="btn" onclick={onRecheck} disabled={statusLoading}>
          {statusLoading ? "Checking…" : "Recheck auth"}
        </button>
      </div>
      <p class="status" class:ok={cliOk} class:bad={status && !cliOk}>
        {#if cliOk}
          ✓ Auth looks good
        {:else}
          Still need a signed-in CLI session
        {/if}
      </p>
    {:else if step === 3}
      <h2 id="setup-title">Choose a project folder</h2>
      <p class="body">
        Prefer a <strong>git repo</strong> (or make a new folder and <code>git init</code>). Grok
        will work inside this path.
      </p>
      <pre class="code path">{cwd || "(none selected)"}</pre>
      <div class="row">
        <button type="button" class="btn primary" onclick={onPickFolder}>Browse…</button>
      </div>
      <p class="status" class:ok={hasFolder}>
        {#if hasFolder}
          ✓ Folder set
        {:else}
          Pick a folder to continue
        {/if}
      </p>
    {:else}
      <h2 id="setup-title">How do you want to start?</h2>
      <p class="body">Pick one — you can always use the others later from the toolbar.</p>
      <div class="choices">
        <button
          type="button"
          class="choice"
          disabled={!cliOk}
          onclick={() => {
            onDone();
            onKickoff();
          }}
        >
          <strong>Guided kickoff</strong>
          <span>Short interview → strong first prompt (best if you’re not sure how to ask)</span>
        </button>
        <button
          type="button"
          class="choice"
          disabled={!cliOk}
          onclick={() => {
            onDone();
            onLoops();
          }}
        >
          <strong>Run a loop</strong>
          <span>Explore → plan → implement → verify → wrap-up (great for shipping)</span>
        </button>
        <button
          type="button"
          class="choice"
          disabled={!cliOk}
          onclick={() => {
            onDone();
            onConnect();
          }}
        >
          <strong>Just chat</strong>
          <span>Connect and free-form like a desktop assistant</span>
        </button>
      </div>
    {/if}

    <div class="actions">
      {#if step > 0 && step < 4}
        <button type="button" class="btn ghost" onclick={back}>Back</button>
      {:else}
        <span></span>
      {/if}
      <div class="right">
        <button type="button" class="btn ghost" onclick={onDone}>Skip setup</button>
        {#if step < 4}
          <button
            type="button"
            class="btn primary"
            onclick={next}
            disabled={(step === 1 || step === 2) && !cliOk && step > 0 ? false : false}
          >
            {step === 3 && !hasFolder ? "Continue anyway" : "Next"}
          </button>
        {:else}
          <button type="button" class="btn primary" onclick={onDone}>Close</button>
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 80;
    background: rgba(0, 0, 0, 0.55);
    display: grid;
    place-items: center;
    padding: 1rem;
  }

  .card {
    width: min(480px, 100%);
    background: #151922;
    border: 1px solid #2a3344;
    border-radius: 14px;
    padding: 1.25rem 1.35rem;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.5);
  }

  .step-label {
    margin: 0;
    font-size: 0.75rem;
    color: #8b93a7;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  h2 {
    margin: 0.35rem 0 0.75rem;
    font-size: 1.15rem;
  }

  .body {
    margin: 0 0 0.65rem;
    font-size: 0.92rem;
    line-height: 1.5;
    color: #c5cad6;
  }

  .muted {
    margin: 0;
    font-size: 0.8rem;
    color: #8b93a7;
  }

  .code {
    margin: 0.4rem 0 0.65rem;
    padding: 0.65rem 0.75rem;
    background: #0d0f12;
    border: 1px solid #232a38;
    border-radius: 8px;
    font-size: 0.78rem;
    overflow: auto;
    color: #a5b4fc;
    white-space: pre-wrap;
  }

  .code.path {
    font-size: 0.8rem;
    color: #c5cad6;
  }

  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-bottom: 0.5rem;
  }

  .status {
    margin: 0.5rem 0 0;
    font-size: 0.85rem;
    color: #8b93a7;
  }

  .status.ok {
    color: #86efac;
  }

  .status.bad {
    color: #fca5a5;
  }

  .choices {
    display: grid;
    gap: 0.45rem;
  }

  .choice {
    text-align: left;
    border: 1px solid #2a3344;
    background: #0d0f12;
    color: #e8eaed;
    border-radius: 10px;
    padding: 0.7rem 0.8rem;
    cursor: pointer;
    display: grid;
    gap: 0.2rem;
  }

  .choice:hover:not(:disabled) {
    border-color: #3b82f6;
    background: #1a2744;
  }

  .choice:disabled {
    opacity: 0.45;
    cursor: default;
  }

  .choice span {
    font-size: 0.8rem;
    color: #8b93a7;
  }

  .actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
    margin-top: 1.15rem;
  }

  .right {
    display: flex;
    gap: 0.45rem;
  }

  .btn {
    border: 1px solid #2a3344;
    background: #1a2030;
    color: #e8eaed;
    border-radius: 8px;
    padding: 0.45rem 0.85rem;
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
    cursor: default;
  }

  code {
    font-family: ui-monospace, Menlo, monospace;
    font-size: 0.85em;
    color: #a5b4fc;
  }
</style>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";

  export type PrivacySnapshot = {
    grokHome: string | null;
    authCachePresent: boolean;
    sessionsDirPresent: boolean;
    telemetryEnabled: boolean | null;
    traceUpload: boolean | null;
    configYolo: boolean | null;
    configPermissionMode: string | null;
    configPath: string | null;
    summaryLines: string[];
  };

  let {
    open = false,
    alwaysApprove = false,
    isGit = true,
    onClose,
    onAck,
    onSafeExplore,
  }: {
    open?: boolean;
    alwaysApprove?: boolean;
    isGit?: boolean;
    onClose: () => void;
    onAck?: () => void;
    onSafeExplore?: () => void;
  } = $props();

  let snap = $state<PrivacySnapshot | null>(null);
  let loading = $state(false);
  let error = $state<string | null>(null);

  async function load() {
    loading = true;
    error = null;
    try {
      snap = await invoke<PrivacySnapshot>("privacy_snapshot");
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    if (open) load();
  });

  async function openPrivacyDocs() {
    try {
      await openUrl("https://x.ai/cli");
    } catch {
      /* ignore */
    }
  }
</script>

{#if open}
  <div class="overlay" role="dialog" aria-modal="true" aria-labelledby="safety-title">
    <div class="card">
      <header>
        <div>
          <p class="eyebrow">Transparency</p>
          <h2 id="safety-title">Data & safety</h2>
        </div>
        <button type="button" class="x" onclick={onClose} aria-label="Close">×</button>
      </header>

      <p class="lead">
        Read this once before exploring other people’s machines of… wait, <em>your</em> machine.
        Coding agents are powerful. Here’s what Grok Desktop does — and what it doesn’t.
      </p>

      <section>
        <h3>What leaves this computer</h3>
        <ul>
          <li>
            <strong>Prompts, answers, and files the agent reads/edits</strong> may go to
            <strong>xAI</strong> via the official Grok Build CLI (required for the model to work).
          </li>
          <li>
            <strong>MCP / plugins</strong> you enabled in <code>~/.grok</code> can call third-party
            services (their policies apply).
          </li>
          <li>
            Optional Grok Build <strong>telemetry / traces</strong> depend on CLI config (see status
            below) and account settings.
          </li>
        </ul>
      </section>

      <section>
        <h3>What stays local</h3>
        <ul>
          <li>This app’s UI preferences (folder, recent sessions list).</li>
          <li>
            Grok sessions under <code>~/.grok/sessions/</code>
            {#if snap?.sessionsDirPresent}
              <span class="ok"> · present</span>
            {/if}
          </li>
          <li>
            Notes only if you click <strong>Export</strong> →
            <code>.grok-desktop/notes/</code> in your project.
          </li>
          <li>No separate “Grok Desktop cloud” account or backend of ours.</li>
        </ul>
      </section>

      <section>
        <h3>Local Grok Build snapshot</h3>
        {#if loading}
          <p class="muted">Reading config…</p>
        {:else if error}
          <p class="bad">{error}</p>
        {:else if snap}
          <dl>
            <div>
              <dt>Auth cache</dt>
              <dd>{snap.authCachePresent ? "present (~/.grok/auth.json)" : "not found"}</dd>
            </div>
            <div>
              <dt>Telemetry (config)</dt>
              <dd>
                {#if snap.telemetryEnabled === null}
                  unknown — check config / TUI /privacy
                {:else}
                  {snap.telemetryEnabled ? "ON" : "OFF"}
                {/if}
              </dd>
            </div>
            <div>
              <dt>Trace upload (config)</dt>
              <dd>
                {#if snap.traceUpload === null}
                  unset / inherit
                {:else}
                  {snap.traceUpload ? "ON" : "OFF"}
                {/if}
              </dd>
            </div>
            <div>
              <dt>CLI permission mode</dt>
              <dd>{snap.configPermissionMode ?? "—"}</dd>
            </div>
            <div>
              <dt>Config file</dt>
              <dd><code>{snap.configPath ?? "—"}</code></dd>
            </div>
          </dl>
          <ul class="summary">
            {#each snap.summaryLines as line}
              <li>{line}</li>
            {/each}
          </ul>
        {/if}
      </section>

      <section>
        <h3>Safer exploring (checklist)</h3>
        <ul class="check">
          <li class:bad={alwaysApprove} class:ok={!alwaysApprove}>
            Auto-approve tools is <strong>{alwaysApprove ? "ON — careful" : "OFF — safer"}</strong>
          </li>
          <li class:bad={!isGit} class:ok={isGit}>
            Project is {isGit ? "a git repo (good)" : "not git — hard to undo edits"}
          </li>
          <li>Use a dedicated project folder, not your whole home directory.</li>
          <li>Don’t leave live API keys in the tree if you can avoid it.</li>
          <li>
            For account retention: open Grok TUI and run <code>/privacy</code>.
          </li>
        </ul>
      </section>

      <section class="actions-block">
        {#if onSafeExplore}
          <button type="button" class="btn accent" onclick={onSafeExplore}>
            Safe explore mode
          </button>
          <p class="muted small">
            Turns auto-approve off and starts a read-heavy “learn this repo” style path.
          </p>
        {/if}
        <div class="row">
          <button type="button" class="btn" onclick={openPrivacyDocs}>x.ai/cli docs</button>
          <button type="button" class="btn" onclick={load} disabled={loading}>Refresh</button>
        </div>
      </section>

      <footer>
        {#if onAck}
          <button
            type="button"
            class="btn primary"
            onclick={() => {
              onAck();
              onClose();
            }}
          >
            I understand — continue
          </button>
        {:else}
          <button type="button" class="btn primary" onclick={onClose}>Close</button>
        {/if}
      </footer>

      <p class="legal">
        Not legal advice. Not affiliated with xAI. Full write-up:
        <code>docs/SAFETY-AND-DATA.md</code> in the repo.
      </p>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 85;
    background: rgba(0, 0, 0, 0.6);
    display: grid;
    place-items: center;
    padding: 1rem;
  }

  .card {
    width: min(520px, 100%);
    max-height: min(92vh, 820px);
    overflow: auto;
    background: #151922;
    border: 1px solid #2a3344;
    border-radius: 14px;
    padding: 1.15rem 1.25rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .eyebrow {
    margin: 0;
    font-size: 0.72rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: #fbbf24;
  }

  h2 {
    margin: 0.2rem 0 0;
    font-size: 1.15rem;
  }

  h3 {
    margin: 0 0 0.4rem;
    font-size: 0.88rem;
    color: #a5b4fc;
  }

  .lead {
    margin: 0.75rem 0;
    font-size: 0.9rem;
    line-height: 1.45;
    color: #c5cad6;
  }

  section {
    margin: 0.9rem 0;
  }

  ul {
    margin: 0;
    padding-left: 1.15rem;
    font-size: 0.86rem;
    line-height: 1.5;
    color: #c5cad6;
  }

  .check li {
    margin: 0.25rem 0;
  }

  .check li.ok {
    color: #86efac;
  }

  .check li.bad {
    color: #fde68a;
  }

  dl {
    margin: 0;
    display: grid;
    gap: 0.35rem;
    font-size: 0.82rem;
  }

  dt {
    color: #8b93a7;
  }

  dd {
    margin: 0.1rem 0 0.25rem;
  }

  .summary {
    margin-top: 0.65rem;
    font-size: 0.8rem;
    color: #8b93a7;
  }

  .x {
    border: none;
    background: transparent;
    color: #8b93a7;
    font-size: 1.4rem;
    cursor: pointer;
  }

  .actions-block {
    border-top: 1px solid #232a38;
    padding-top: 0.75rem;
  }

  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-top: 0.5rem;
  }

  footer {
    display: flex;
    justify-content: flex-end;
    margin-top: 0.75rem;
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

  .btn.accent {
    background: #0f766e;
    border-color: #14b8a6;
  }

  .btn:disabled {
    opacity: 0.5;
  }

  .muted {
    color: #8b93a7;
  }

  .small {
    font-size: 0.78rem;
    margin: 0.35rem 0 0;
  }

  .ok {
    color: #86efac;
  }

  .bad {
    color: #fca5a5;
  }

  .legal {
    margin: 0.75rem 0 0;
    font-size: 0.72rem;
    color: #6b7280;
    line-height: 1.4;
  }

  code {
    font-family: ui-monospace, Menlo, monospace;
    font-size: 0.85em;
    color: #a5b4fc;
  }
</style>

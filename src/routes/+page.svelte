<script lang="ts">
  import { onMount } from "svelte";
  import { fetchGrokStatus } from "$lib/grok";
  import type { GrokStatus } from "$lib/types";

  let status = $state<GrokStatus | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  async function refresh() {
    loading = true;
    error = null;
    try {
      status = await fetchGrokStatus();
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      status = null;
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    refresh();
  });
</script>

<div class="app">
  <header class="header">
    <div class="brand">
      <span class="mark" aria-hidden="true">◈</span>
      <div>
        <h1>Grok Desktop</h1>
        <p class="tagline">Claude Desktop–style shell for Grok Build (ACP)</p>
      </div>
    </div>
    <button type="button" class="btn" onclick={refresh} disabled={loading}>
      {loading ? "Checking…" : "Refresh"}
    </button>
  </header>

  <main class="main">
    <section class="card" aria-labelledby="status-heading">
      <h2 id="status-heading">Grok Build status</h2>

      {#if loading && !status}
        <p class="muted">Probing local `grok` CLI…</p>
      {:else if error}
        <p class="bad">{error}</p>
      {:else if status}
        <div class="status-row">
          <span
            class="dot"
            class:ok={status.ready}
            class:warn={!status.ready}
            title={status.ready ? "Ready" : "Not ready"}
          ></span>
          <p class="message">{status.message}</p>
        </div>

        <dl class="meta">
          <div>
            <dt>Binary</dt>
            <dd><code>{status.binaryPath ?? "—"}</code></dd>
          </div>
          <div>
            <dt>Version</dt>
            <dd><code>{status.version ?? "—"}</code></dd>
          </div>
          <div>
            <dt>Authenticated</dt>
            <dd>{status.authenticated ? "Yes" : "No"}</dd>
          </div>
        </dl>
      {/if}
    </section>

    <section class="card muted-card">
      <h2>Roadmap (MVP)</h2>
      <ol class="roadmap">
        <li class="done">Locate `grok` + auth soft-check</li>
        <li>Spawn <code>grok agent stdio</code> (ACP)</li>
        <li>Stream message / thought / tool / plan updates</li>
        <li>Permission approve / deny</li>
        <li>Project folder + session list</li>
      </ol>
      <p class="muted small">
        Agent runtime, tools, sandbox, and login stay in the official CLI. This
        app is a thin client — see <code>docs/PRODUCT.md</code>.
      </p>
    </section>
  </main>
</div>

<style>
  :global(html, body) {
    margin: 0;
    min-height: 100%;
  }

  :global(body) {
    font-family:
      "Inter",
      system-ui,
      -apple-system,
      Segoe UI,
      Roboto,
      sans-serif;
    color: #e8eaed;
    background: #0d0f12;
    -webkit-font-smoothing: antialiased;
  }

  .app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid #1e2430;
    background: #12151a;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .mark {
    font-size: 1.5rem;
    color: #7dd3fc;
  }

  h1 {
    margin: 0;
    font-size: 1.15rem;
    font-weight: 600;
    letter-spacing: -0.02em;
  }

  .tagline {
    margin: 0.15rem 0 0;
    font-size: 0.8rem;
    color: #8b93a7;
  }

  .main {
    flex: 1;
    padding: 1.25rem;
    display: grid;
    gap: 1rem;
    max-width: 720px;
    width: 100%;
    margin: 0 auto;
    box-sizing: border-box;
  }

  .card {
    background: #151922;
    border: 1px solid #232a38;
    border-radius: 12px;
    padding: 1.1rem 1.2rem;
  }

  .muted-card {
    background: #12151a;
  }

  h2 {
    margin: 0 0 0.75rem;
    font-size: 0.95rem;
    font-weight: 600;
    color: #c5cad6;
  }

  .status-row {
    display: flex;
    align-items: flex-start;
    gap: 0.65rem;
  }

  .dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    margin-top: 0.35rem;
    flex-shrink: 0;
    background: #6b7280;
  }

  .dot.ok {
    background: #34d399;
    box-shadow: 0 0 8px rgba(52, 211, 153, 0.45);
  }

  .dot.warn {
    background: #fbbf24;
  }

  .message {
    margin: 0;
    line-height: 1.45;
  }

  .meta {
    margin: 1rem 0 0;
    display: grid;
    gap: 0.55rem;
  }

  .meta div {
    display: grid;
    grid-template-columns: 7.5rem 1fr;
    gap: 0.5rem;
    font-size: 0.85rem;
  }

  dt {
    color: #8b93a7;
    margin: 0;
  }

  dd {
    margin: 0;
    word-break: break-all;
  }

  code {
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.8em;
    color: #a5b4fc;
  }

  .roadmap {
    margin: 0;
    padding-left: 1.2rem;
    line-height: 1.7;
    font-size: 0.9rem;
    color: #c5cad6;
  }

  .roadmap .done {
    color: #34d399;
  }

  .muted {
    color: #8b93a7;
  }

  .small {
    font-size: 0.8rem;
    margin: 0.85rem 0 0;
    line-height: 1.45;
  }

  .bad {
    color: #f87171;
    margin: 0;
  }

  .btn {
    border: 1px solid #2a3344;
    background: #1a2030;
    color: #e8eaed;
    border-radius: 8px;
    padding: 0.45rem 0.9rem;
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
  }

  .btn:hover:not(:disabled) {
    border-color: #3b82f6;
    background: #1e293b;
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: default;
  }
</style>

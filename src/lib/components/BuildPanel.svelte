<script lang="ts">
  import type { BuildMonitor } from "$lib/build";

  let {
    open = false,
    report = null as BuildMonitor | null,
    loading = false,
    updating = false,
    onClose,
    onRefresh,
    onUpdate,
  }: {
    open?: boolean;
    report?: BuildMonitor | null;
    loading?: boolean;
    updating?: boolean;
    onClose: () => void;
    onRefresh: () => void;
    onUpdate: () => void;
  } = $props();
</script>

{#if open}
  <div class="overlay" role="dialog" aria-modal="true" aria-labelledby="bm-title">
    <div class="card">
      <header>
        <div>
          <p class="eyebrow">Grok Build monitor</p>
          <h2 id="bm-title">CLI · updates · models</h2>
        </div>
        <button type="button" class="x" onclick={onClose} aria-label="Close">×</button>
      </header>

      {#if loading && !report}
        <p class="muted">Reading local Grok install…</p>
      {:else if report}
        <section class="grid">
          <div>
            <h3>Status</h3>
            <p class:ok={report.ready} class:bad={!report.ready}>
              {report.ready ? "Ready" : "Not ready"} — {report.message}
            </p>
            <dl>
              <div><dt>Binary</dt><dd><code>{report.binaryPath ?? "—"}</code></dd></div>
              <div><dt>Version</dt><dd><code>{report.version ?? "—"}</code></dd></div>
              <div><dt>Home</dt><dd><code>{report.grokHome ?? "—"}</code></dd></div>
            </dl>
          </div>

          <div>
            <h3>Updates</h3>
            {#if report.update.updateAvailable}
              <p class="warn">
                Update available: {report.update.currentVersion} → {report.update.latestVersion}
              </p>
              <button type="button" class="btn primary" onclick={onUpdate} disabled={updating}>
                {updating ? "Updating…" : "Run grok update"}
              </button>
            {:else}
              <p class="ok">
                Up to date
                {#if report.update.latestVersion}
                  ({report.update.latestVersion})
                {/if}
              </p>
              <button type="button" class="btn" onclick={onUpdate} disabled={updating}>
                {updating ? "Updating…" : "Force check / update"}
              </button>
            {/if}
            {#if report.update.channel}
              <p class="muted small">Channel: {report.update.channel}
                {#if report.update.autoUpdate != null}
                  · auto_update={String(report.update.autoUpdate)}
                {/if}
              </p>
            {/if}
            {#if report.update.error}
              <p class="bad small">{report.update.error}</p>
            {/if}
          </div>
        </section>

        <section>
          <h3>Models (cache)</h3>
          {#if report.models.length === 0}
            <p class="muted">No models cache yet — connect once or run <code>grok models</code>.</p>
          {:else}
            <ul class="models">
              {#each report.models as m}
                <li>
                  <strong>{m.name}</strong>
                  <code>{m.id}</code>
                  {#if m.contextWindow}
                    <span class="muted">{m.contextWindow.toLocaleString()} ctx</span>
                  {/if}
                  {#if m.description}
                    <span class="desc">{m.description}</span>
                  {/if}
                </li>
              {/each}
            </ul>
          {/if}
        </section>

        {#if report.changelogPreview.length}
          <section>
            <h3>Changelog (local)</h3>
            <ul class="log">
              {#each report.changelogPreview as line}
                <li>{line}</li>
              {/each}
            </ul>
          </section>
        {/if}
      {/if}

      <footer>
        <button type="button" class="btn" onclick={onRefresh} disabled={loading}>
          {loading ? "…" : "Refresh"}
        </button>
        <button type="button" class="btn primary" onclick={onClose}>Done</button>
      </footer>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 75;
    background: rgba(0, 0, 0, 0.55);
    display: grid;
    place-items: center;
    padding: 1rem;
  }

  .card {
    width: min(560px, 100%);
    max-height: min(90vh, 800px);
    overflow: auto;
    background: #151922;
    border: 1px solid #2a3344;
    border-radius: 14px;
    padding: 1.1rem 1.25rem;
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
    color: #7dd3fc;
  }

  h2 {
    margin: 0.2rem 0 0;
    font-size: 1.1rem;
  }

  h3 {
    margin: 0 0 0.4rem;
    font-size: 0.85rem;
    color: #a5b4fc;
  }

  .x {
    border: none;
    background: transparent;
    color: #8b93a7;
    font-size: 1.4rem;
    cursor: pointer;
  }

  .grid {
    display: grid;
    gap: 1rem;
    margin: 0.85rem 0;
  }

  @media (min-width: 520px) {
    .grid {
      grid-template-columns: 1fr 1fr;
    }
  }

  section {
    margin-bottom: 0.85rem;
  }

  dl {
    margin: 0.4rem 0 0;
    display: grid;
    gap: 0.35rem;
    font-size: 0.8rem;
  }

  dt {
    color: #8b93a7;
  }

  dd {
    margin: 0.1rem 0 0;
    word-break: break-all;
  }

  .models {
    list-style: none;
    margin: 0;
    padding: 0;
    display: grid;
    gap: 0.45rem;
  }

  .models li {
    border: 1px solid #232a38;
    border-radius: 8px;
    padding: 0.45rem 0.55rem;
    display: grid;
    gap: 0.15rem;
    font-size: 0.82rem;
  }

  .desc {
    color: #8b93a7;
    font-size: 0.78rem;
  }

  .log {
    margin: 0;
    padding-left: 1.1rem;
    font-size: 0.78rem;
    color: #c5cad6;
  }

  footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.45rem;
    margin-top: 0.75rem;
  }

  .btn {
    border: 1px solid #2a3344;
    background: #1a2030;
    color: #e8eaed;
    border-radius: 8px;
    padding: 0.4rem 0.8rem;
    font-size: 0.85rem;
    cursor: pointer;
  }

  .btn.primary {
    background: #1d4ed8;
    border-color: #2563eb;
  }

  .btn:disabled {
    opacity: 0.5;
  }

  .ok {
    color: #86efac;
  }
  .bad {
    color: #fca5a5;
  }
  .warn {
    color: #fde68a;
  }
  .muted {
    color: #8b93a7;
  }
  .small {
    font-size: 0.78rem;
  }

  code {
    font-family: ui-monospace, Menlo, monospace;
    font-size: 0.85em;
    color: #a5b4fc;
  }
</style>

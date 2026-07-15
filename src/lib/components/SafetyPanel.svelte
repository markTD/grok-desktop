<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";

  export type PrivacySnapshot = {
    grokHome: string | null;
    authCachePresent: boolean;
    sessionsDirPresent: boolean;
    telemetryEnabled: boolean | null;
    traceUpload: boolean | null;
    /** off | on | unknown */
    analyticsStatus: string;
    analyticsLabel: string;
    analyticsDetail: string;
    configYolo: boolean | null;
    configPermissionMode: string | null;
    configPath: string | null;
    summaryLines: string[];
  };

  let {
    open = false,
    alwaysApprove = false,
    isGit = true,
    connected = false,
    onClose,
    onAck,
    onSafeExplore,
  }: {
    open?: boolean;
    alwaysApprove?: boolean;
    isGit?: boolean;
    connected?: boolean;
    onClose: () => void;
    onAck?: () => void;
    onSafeExplore?: () => void;
  } = $props();

  let tab = $state<"status" | "guardrails" | "controls">("status");
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
    if (open) {
      tab = "status";
      load();
    }
  });

  async function openCliDocs() {
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
          <p class="eyebrow">Trust center</p>
          <h2 id="safety-title">Data status · harness · guardrails</h2>
        </div>
        <button type="button" class="x" onclick={onClose} aria-label="Close">×</button>
      </header>

      <p class="lead">
        Plain answers for normal people. <strong>Grok Desktop does not run its own cloud.</strong>
        The AI still needs the internet through the official <strong>Grok Build</strong> app from
        xAI — so some project content can leave your computer when the agent works.
      </p>

      <nav class="tabs" aria-label="Safety sections">
        <button type="button" class:on={tab === "status"} onclick={() => (tab = "status")}>
          1. Data status
        </button>
        <button
          type="button"
          class:on={tab === "guardrails"}
          onclick={() => (tab = "guardrails")}
        >
          2. Harness & guardrails
        </button>
        <button type="button" class:on={tab === "controls"} onclick={() => (tab = "controls")}>
          3. Your controls
        </button>
      </nav>

      {#if tab === "status"}
        <section>
          <h3>Will it send my whole codebase?</h3>
          <div class="answer-card yellow">
            <div class="badge">Not all at once · not by default as a zip</div>
            <p>
              There is <strong>no “upload entire project” button</strong> in Grok Desktop.
              When you chat, Grok Build sends what the <strong>model needs for that turn</strong>:
              your messages, plus <strong>files and tool results it chooses to read</strong>
              (and edits it makes). Over a long session, that can cover a large part of a project
              — treat it like a remote coworker who can open files you allow tools to touch.
            </p>
          </div>

          <h3 class="mt">Live status board</h3>
          <div class="board">
            <article class="row-card red">
              <header>
                <span class="dot"></span>
                <strong>Your chat &amp; answers</strong>
                <em>Goes to xAI</em>
              </header>
              <p>
                Required for the model to work. Same idea as using grok.com or the Grok Build TUI.
              </p>
            </article>

            <article class="row-card red">
              <header>
                <span class="dot"></span>
                <strong>Files the agent reads or edits</strong>
                <em>Can go to xAI</em>
              </header>
              <p>
                When tools open <code>src/…</code>, configs, logs, etc., that content may be
                included in model traffic. Secrets in the project can leak this way.
              </p>
            </article>

            <article class="row-card yellow">
              <header>
                <span class="dot"></span>
                <strong>Shell commands &amp; tool output</strong>
                <em>Can go to xAI</em>
              </header>
              <p>
                Command results the agent runs (tests, git status, etc.) can be sent back into the
                model context.
              </p>
            </article>

            <article class="row-card yellow">
              <header>
                <span class="dot"></span>
                <strong>MCP / plugins (if enabled)</strong>
                <em>Third parties</em>
              </header>
              <p>
                Anything under <code>~/.grok</code> plugins/MCP (Vercel, Sentry, …) can send data
                to those services. Review what you installed.
              </p>
            </article>

            <!-- One card — not "telemetry" + "traces" jargon -->
            <article
              class="row-card"
              class:green={snap?.analyticsStatus === "off"}
              class:yellow={snap?.analyticsStatus === "unknown" || !snap}
              class:red={snap?.analyticsStatus === "on"}
            >
              <header>
                <span class="dot"></span>
                <strong>Optional product analytics</strong>
                <em>{snap?.analyticsLabel ?? "…"}</em>
              </header>
              <p>
                {#if snap}
                  {snap.analyticsDetail}
                {:else if loading}
                  Checking your Grok config…
                {:else}
                  Could not read analytics settings yet. Tap Refresh.
                {/if}
              </p>
              <p class="subnote">
                <strong>Not the same as AI chat.</strong> Chat still needs the internet so Grok can
                answer. “Product analytics” means extra usage stats about the tool itself (crashes,
                feature counts, etc.) if the Grok CLI is set to send them.
              </p>
            </article>

            <article class="row-card green">
              <header>
                <span class="dot"></span>
                <strong>Grok Desktop “cloud product”</strong>
                <em>None</em>
              </header>
              <p>
                This open-source app does not run a backend that receives your repo. No separate
                Grok Desktop account.
              </p>
            </article>

            <article class="row-card green">
              <header>
                <span class="dot"></span>
                <strong>Chat history on disk (local)</strong>
                <em>Stays here</em>
              </header>
              <p>
                Official CLI stores sessions under <code>~/.grok/sessions/</code>
                {#if snap?.sessionsDirPresent}<span class="pill">found</span>{/if}. Export notes
                only if you click Export.
              </p>
            </article>
          </div>

          {#if loading}
            <p class="muted">Refreshing local config…</p>
          {:else if error}
            <p class="bad">{error}</p>
          {/if}

          <p class="note">
            <strong>How long xAI keeps your coding chats</strong> is an
            <strong>account</strong> setting, not a line in this app. Open Terminal, run
            <code>grok</code>, type <code>/privacy</code>, and use what xAI shows for your plan.
            That screen is the authority for retention — we only report what’s in local config for
            optional analytics.
          </p>
        </section>
      {:else if tab === "guardrails"}
        <section>
          <h3>Three layers of the harness</h3>
          <p class="muted small">
            Think of a seatbelt + airbag + driver: each layer does a different job. None of them
            make the AI “offline.”
          </p>

          <div class="layers">
            <article class="layer">
              <h4>1 · Grok Desktop (this app)</h4>
              <ul>
                <li>Shows you tools, thoughts, and status instead of a blank terminal.</li>
                <li>Default: <strong>auto-approve off</strong> (you stay in the loop when the agent asks).</li>
                <li>Warns if the folder is <strong>not git</strong> and auto-approve is on.</li>
                <li>Guided setup + Safety screen so first-time users see data facts first.</li>
                <li>Export notes only on purpose; no silent “desktop cloud” upload of our own.</li>
              </ul>
              <p class="layer-lim">
                Limit: we cannot stop xAI from receiving model traffic once you Connect. We only
                wrap the official CLI.
              </p>
            </article>

            <article class="layer">
              <h4>2 · Grok Build CLI (xAI harness)</h4>
              <ul>
                <li>Real agent: file tools, shell, web, MCP, subagents, plan mode, sandbox options.</li>
                <li>Permission modes, deny rules, hooks (see CLI docs under <code>~/.grok/docs</code>).</li>
                <li>Sessions, auth, updates, <code>/privacy</code> live here.</li>
              </ul>
              <p class="layer-lim">
                Limit: this is still an online coding agent. Tooling is powerful by design.
              </p>
            </article>

            <article class="layer">
              <h4>3 · You (project &amp; habits)</h4>
              <ul>
                <li>Pick a <strong>small git project</strong>, not your whole home folder.</li>
                <li>Keep secrets out of the tree when you can.</li>
                <li>Start with <strong>Learn / plan / explore</strong> before “ship everything.”</li>
                <li>Turn auto-approve on only when you accept the risk.</li>
              </ul>
            </article>
          </div>

          <h3 class="mt">Right now on this machine</h3>
          <div class="now">
            <div class:ok={!alwaysApprove} class:warn={alwaysApprove}>
              Auto-approve tools: <strong>{alwaysApprove ? "ON" : "OFF"}</strong>
              {alwaysApprove ? " — agent can run tools without asking (as configured)" : " — safer for learning"}
            </div>
            <div class:ok={isGit} class:warn={!isGit}>
              Project git: <strong>{isGit ? "yes" : "no"}</strong>
              {isGit ? " — you can diff / revert" : " — hard to undo mistakes"}
            </div>
            <div class:ok={connected} class:muted={!connected}>
              Agent session: <strong>{connected ? "connected" : "not connected"}</strong>
              {connected ? " — model traffic can flow when you chat" : " — nothing sent until you Connect"}
            </div>
          </div>
        </section>
      {:else}
        <section>
          <h3>What you can do in 60 seconds</h3>
          <ol class="steps">
            <li>
              <strong>Keep auto-approve off</strong> until you trust a project (More → Session
              controls).
            </li>
            <li>
              <strong>Use a git folder</strong> — <code>git init</code> if needed.
            </li>
            <li>
              <strong>Account privacy:</strong> open Terminal → <code>grok</code> → type
              <code>/privacy</code> and set retention the way xAI offers for your plan.
            </li>
            <li>
              <strong>Optional product analytics OFF:</strong> edit
              <code>~/.grok/config.toml</code>, under <code>[features]</code> set
              <code>telemetry = false</code>, save, then <strong>Refresh status</strong> here.
              (Does not stop AI chat from using the network.)
            </li>
            <li>
              <strong>Review MCP/plugins</strong> you enabled — they can call outside services.
            </li>
          </ol>

          <div class="config-box">
            <h4>Optional analytics status (simple)</h4>
            {#if loading}
              <p class="muted">Loading…</p>
            {:else if snap}
              <p class="analytics-big" class:off={snap.analyticsStatus === "off"} class:on={snap.analyticsStatus === "on"} class:unk={snap.analyticsStatus === "unknown"}>
                {snap.analyticsLabel}
              </p>
              <p class="muted small">{snap.analyticsDetail}</p>
              <p class="muted small mt">
                Tech names in the file (if you care): telemetry =
                {snap.telemetryEnabled === null ? "?" : String(snap.telemetryEnabled)},
                trace_upload =
                {snap.traceUpload === null ? "?" : String(snap.traceUpload)}. File:
                <code>{snap.configPath ?? "—"}</code>
              </p>
              <p class="muted small">
                Signed in locally: {snap.authCachePresent ? "yes" : "no"} · Permission mode:
                {snap.configPermissionMode ?? "not set"}
              </p>
            {:else}
              <p class="muted">No snapshot yet.</p>
            {/if}
          </div>

          <div class="actions-block">
            {#if onSafeExplore}
              <button type="button" class="btn accent" onclick={onSafeExplore}>
                Safe explore mode
              </button>
              <p class="muted small">
                Forces auto-approve off and points you at a read-heavy Learn loop.
              </p>
            {/if}
            <div class="row">
              <button type="button" class="btn" onclick={openCliDocs}>Official CLI docs</button>
              <button type="button" class="btn" onclick={load} disabled={loading}>
                Refresh status
              </button>
            </div>
          </div>
        </section>
      {/if}

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
            I understand the data status — continue
          </button>
        {:else}
          <button type="button" class="btn primary" onclick={onClose}>Close</button>
        {/if}
      </footer>

      <p class="legal">
        Not legal advice · Not affiliated with xAI · Full doc:
        <code>docs/SAFETY-AND-DATA.md</code>
      </p>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 85;
    background: rgba(0, 0, 0, 0.62);
    display: grid;
    place-items: center;
    padding: 0.75rem;
  }

  .card {
    width: min(560px, 100%);
    max-height: min(94vh, 860px);
    overflow: auto;
    background: #12151a;
    border: 1px solid #2a3344;
    border-radius: 14px;
    padding: 1.1rem 1.2rem 1rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .eyebrow {
    margin: 0;
    font-size: 0.72rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: #fbbf24;
  }

  h2 {
    margin: 0.2rem 0 0;
    font-size: 1.12rem;
  }

  h3 {
    margin: 0 0 0.5rem;
    font-size: 0.95rem;
    color: #e8eaed;
  }

  h3.mt {
    margin-top: 1rem;
  }

  h4 {
    margin: 0 0 0.35rem;
    font-size: 0.88rem;
    color: #a5b4fc;
  }

  .lead {
    margin: 0.65rem 0 0.75rem;
    font-size: 0.9rem;
    line-height: 1.5;
    color: #c5cad6;
  }

  .tabs {
    display: flex;
    flex-wrap: wrap;
    gap: 0.35rem;
    margin-bottom: 0.85rem;
  }

  .tabs button {
    border: 1px solid #2a3344;
    background: #0d0f12;
    color: #8b93a7;
    border-radius: 999px;
    padding: 0.35rem 0.7rem;
    font-size: 0.78rem;
    cursor: pointer;
  }

  .tabs button.on {
    background: #1a2744;
    border-color: #3b82f6;
    color: #e8eaed;
  }

  .answer-card {
    border-radius: 10px;
    padding: 0.75rem 0.85rem;
    border: 1px solid #854d0e;
    background: #2a2110;
  }

  .answer-card p {
    margin: 0.45rem 0 0;
    font-size: 0.86rem;
    line-height: 1.5;
    color: #fde68a;
  }

  .badge {
    display: inline-block;
    font-size: 0.72rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: #fbbf24;
  }

  .board {
    display: grid;
    gap: 0.45rem;
  }

  .row-card {
    border: 1px solid #2a3344;
    border-radius: 10px;
    padding: 0.55rem 0.7rem;
    background: #0d0f12;
  }

  .row-card header {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.4rem;
    margin-bottom: 0.25rem;
  }

  .row-card strong {
    font-size: 0.86rem;
  }

  .row-card em {
    margin-left: auto;
    font-style: normal;
    font-size: 0.72rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    color: #8b93a7;
  }

  .row-card p {
    margin: 0;
    font-size: 0.8rem;
    line-height: 1.45;
    color: #a8b0c0;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #6b7280;
    flex-shrink: 0;
  }

  .row-card.red {
    border-color: #7f1d1d;
    background: #1c1010;
  }
  .row-card.red .dot,
  .row-card.red em {
    background: #f87171;
    color: #fca5a5;
  }
  .row-card.red .dot {
    background: #f87171;
  }

  .row-card.yellow {
    border-color: #854d0e;
    background: #1a160c;
  }
  .row-card.yellow .dot {
    background: #fbbf24;
  }
  .row-card.yellow em {
    color: #fde68a;
  }

  .row-card.green {
    border-color: #166534;
    background: #0c1610;
  }
  .row-card.green .dot {
    background: #34d399;
  }
  .row-card.green em {
    color: #86efac;
  }

  .pill {
    font-size: 0.7rem;
    margin-left: 0.35rem;
    color: #86efac;
  }

  .subnote {
    margin: 0.4rem 0 0;
    font-size: 0.78rem;
    line-height: 1.4;
    color: #8b93a7;
  }

  .analytics-big {
    margin: 0.25rem 0;
    font-size: 1.35rem;
    font-weight: 800;
    letter-spacing: 0.04em;
  }

  .analytics-big.off {
    color: #86efac;
  }

  .analytics-big.on {
    color: #fca5a5;
  }

  .analytics-big.unk {
    color: #fde68a;
  }

  .mt {
    margin-top: 0.45rem !important;
  }

  .note {
    margin: 0.85rem 0 0;
    font-size: 0.8rem;
    line-height: 1.45;
    color: #8b93a7;
    border-left: 3px solid #3b82f6;
    padding-left: 0.65rem;
  }

  .layers {
    display: grid;
    gap: 0.55rem;
  }

  .layer {
    border: 1px solid #2a3344;
    border-radius: 10px;
    padding: 0.65rem 0.75rem;
    background: #0d0f12;
  }

  .layer ul {
    margin: 0.25rem 0;
    padding-left: 1.1rem;
    font-size: 0.84rem;
    line-height: 1.45;
    color: #c5cad6;
  }

  .layer-lim {
    margin: 0.4rem 0 0;
    font-size: 0.78rem;
    color: #8b93a7;
  }

  .now {
    display: grid;
    gap: 0.35rem;
    font-size: 0.84rem;
  }

  .now div {
    padding: 0.45rem 0.6rem;
    border-radius: 8px;
    border: 1px solid #2a3344;
    background: #0d0f12;
    color: #c5cad6;
  }

  .now .ok {
    border-color: #166534;
    color: #bbf7d0;
  }

  .now .warn {
    border-color: #854d0e;
    color: #fde68a;
  }

  .steps {
    margin: 0;
    padding-left: 1.2rem;
    font-size: 0.86rem;
    line-height: 1.55;
    color: #c5cad6;
  }

  .config-box {
    margin-top: 0.85rem;
    padding: 0.65rem 0.75rem;
    border-radius: 10px;
    border: 1px solid #2a3344;
    background: #0d0f12;
  }

  .actions-block {
    margin-top: 0.85rem;
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
    margin-top: 0.9rem;
  }

  .x {
    border: none;
    background: transparent;
    color: #8b93a7;
    font-size: 1.4rem;
    cursor: pointer;
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

  .bad {
    color: #fca5a5;
  }

  .legal {
    margin: 0.7rem 0 0;
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

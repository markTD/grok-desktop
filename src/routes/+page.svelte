<script lang="ts">
  import { onDestroy, onMount, tick } from "svelte";
  import { fetchGrokStatus } from "$lib/grok";
  import {
    acpConnect,
    acpDisconnect,
    acpPrompt,
    contentText,
    onAcpError,
    onAcpLog,
    onAcpStatus,
    onAcpUpdate,
  } from "$lib/acp";
  import type { ChatItem, GrokStatus } from "$lib/types";
  import type { UnlistenFn } from "@tauri-apps/api/event";

  let status = $state<GrokStatus | null>(null);
  let statusLoading = $state(true);

  let cwd = $state("");
  let connected = $state(false);
  let sessionId = $state<string | null>(null);
  let modelId = $state<string | null>(null);
  let connectionMessage = $state("");
  let connecting = $state(false);
  let busy = $state(false);

  let prompt = $state("");
  let items = $state<ChatItem[]>([]);
  let error = $state<string | null>(null);
  let logs = $state<string[]>([]);
  let showLogs = $state(false);

  let scrollEl: HTMLElement | undefined = $state();
  let unlisteners: UnlistenFn[] = [];

  let idSeq = 0;
  function nid(prefix: string) {
    idSeq += 1;
    return `${prefix}-${idSeq}`;
  }

  /** Active streaming assistant / thought row indexes (append chunks). */
  let streamAssistantId: string | null = null;
  let streamThoughtId: string | null = null;

  async function refreshStatus() {
    statusLoading = true;
    try {
      status = await fetchGrokStatus();
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      statusLoading = false;
    }
  }

  async function scrollToBottom() {
    await tick();
    if (scrollEl) scrollEl.scrollTop = scrollEl.scrollHeight;
  }

  function pushSystem(text: string) {
    items = [...items, { id: nid("sys"), role: "system", text }];
    scrollToBottom();
  }

  function handleUpdate(kind: string, update: Record<string, unknown>) {
    const text = contentText(update);

    if (kind === "user_message_chunk") {
      // We already show the local user message; skip echo or soft-merge.
      return;
    }

    if (kind === "agent_message_chunk") {
      if (streamAssistantId) {
        items = items.map((it) =>
          it.id === streamAssistantId && it.role === "assistant"
            ? { ...it, text: it.text + text }
            : it,
        );
      } else {
        const id = nid("asst");
        streamAssistantId = id;
        items = [...items, { id, role: "assistant", text }];
      }
      scrollToBottom();
      return;
    }

    if (kind === "agent_thought_chunk") {
      if (streamThoughtId) {
        items = items.map((it) =>
          it.id === streamThoughtId && it.role === "thought"
            ? { ...it, text: it.text + text }
            : it,
        );
      } else {
        const id = nid("th");
        streamThoughtId = id;
        items = [...items, { id, role: "thought", text }];
      }
      scrollToBottom();
      return;
    }

    if (kind === "tool_call" || kind === "tool_call_update") {
      streamAssistantId = null;
      streamThoughtId = null;
      const title =
        (typeof update.title === "string" && update.title) ||
        (typeof update.toolName === "string" && update.toolName) ||
        "tool";
      const toolStatus =
        (typeof update.status === "string" && update.status) || kind;
      const detail =
        typeof update.kind === "string" ? update.kind : undefined;
      items = [
        ...items,
        {
          id: nid("tool"),
          role: "tool",
          title,
          status: toolStatus,
          detail,
        },
      ];
      scrollToBottom();
      return;
    }

    if (kind === "plan") {
      pushSystem("Plan update received");
      return;
    }

    // ignore available_commands_update and other chatter
  }

  async function connect() {
    error = null;
    connecting = true;
    streamAssistantId = null;
    streamThoughtId = null;
    try {
      const res = await acpConnect(cwd.trim());
      connected = true;
      sessionId = res.sessionId;
      modelId = res.modelId;
      connectionMessage = `Session ${res.sessionId.slice(0, 8)}…`;
      items = [];
      pushSystem(
        `Connected to Grok Build${res.modelId ? ` · ${res.modelId}` : ""} · auto-approve on`,
      );
    } catch (e) {
      connected = false;
      sessionId = null;
      error = e instanceof Error ? e.message : String(e);
    } finally {
      connecting = false;
    }
  }

  async function disconnect() {
    error = null;
    try {
      await acpDisconnect();
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      connected = false;
      sessionId = null;
      modelId = null;
      connectionMessage = "";
      busy = false;
      pushSystem("Disconnected");
    }
  }

  async function sendPrompt() {
    const text = prompt.trim();
    if (!text || !connected || busy) return;

    error = null;
    busy = true;
    streamAssistantId = null;
    streamThoughtId = null;
    items = [...items, { id: nid("user"), role: "user", text }];
    prompt = "";
    await scrollToBottom();

    try {
      const result = await acpPrompt(text);
      streamAssistantId = null;
      streamThoughtId = null;
      if (result.stopReason) {
        pushSystem(`Turn complete · ${result.stopReason}`);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      pushSystem(`Error: ${error}`);
    } finally {
      busy = false;
      streamAssistantId = null;
      streamThoughtId = null;
    }
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      sendPrompt();
    }
  }

  onMount(async () => {
    await refreshStatus();
    unlisteners = await Promise.all([
      onAcpUpdate((ev) => handleUpdate(ev.kind, ev.update)),
      onAcpStatus((ev) => {
        connected = ev.connected;
        sessionId = ev.sessionId;
        connectionMessage = ev.message;
      }),
      onAcpError((msg) => {
        error = msg;
        logs = [...logs.slice(-80), `error: ${msg}`];
      }),
      onAcpLog((msg) => {
        logs = [...logs.slice(-80), msg];
      }),
    ]);
  });

  onDestroy(() => {
    for (const u of unlisteners) u();
    if (connected) {
      acpDisconnect().catch(() => {});
    }
  });
</script>

<div class="app">
  <header class="header">
    <div class="brand">
      <span class="mark" aria-hidden="true">◈</span>
      <div>
        <h1>Grok Desktop</h1>
        <p class="tagline">
          {#if connected}
            {connectionMessage || "Connected"}
            {#if modelId}<span class="pill">{modelId}</span>{/if}
          {:else if status?.ready}
            CLI ready — connect a project
          {:else}
            {status?.message ?? "Checking Grok CLI…"}
          {/if}
        </p>
      </div>
    </div>
    <div class="header-actions">
      <button type="button" class="btn ghost" onclick={refreshStatus} disabled={statusLoading}>
        CLI
      </button>
      <button type="button" class="btn ghost" onclick={() => (showLogs = !showLogs)}>
        Logs
      </button>
    </div>
  </header>

  <section class="toolbar">
    <label class="cwd-label" for="cwd">Project folder</label>
    <input
      id="cwd"
      class="cwd"
      bind:value={cwd}
      disabled={connected || connecting}
      placeholder="/absolute/path/to/project"
      spellcheck="false"
    />
    {#if connected}
      <button type="button" class="btn danger" onclick={disconnect} disabled={busy}>
        Disconnect
      </button>
    {:else}
      <button
        type="button"
        class="btn primary"
        onclick={connect}
        disabled={connecting || !status?.ready}
      >
        {connecting ? "Connecting…" : "Connect"}
      </button>
    {/if}
  </section>

  {#if error}
    <div class="banner bad" role="alert">{error}</div>
  {/if}

  {#if showLogs}
    <section class="logs" aria-label="Debug log">
      {#if logs.length === 0}
        <p class="muted">No log lines yet.</p>
      {:else}
        {#each logs as line}
          <div class="log-line">{line}</div>
        {/each}
      {/if}
    </section>
  {/if}

  <main class="chat" bind:this={scrollEl} aria-live="polite">
    {#if items.length === 0}
      <div class="empty">
        <p>
          Connect to start an ACP session with <code>grok agent stdio</code>.
        </p>
        <p class="muted">
          v1 uses <strong>auto-approve</strong> for tools. Permission UI comes next.
        </p>
      </div>
    {:else}
      {#each items as item (item.id)}
        {#if item.role === "user"}
          <article class="bubble user">
            <header>You</header>
            <pre>{item.text}</pre>
          </article>
        {:else if item.role === "assistant"}
          <article class="bubble assistant">
            <header>Grok</header>
            <pre>{item.text}</pre>
          </article>
        {:else if item.role === "thought"}
          <article class="bubble thought">
            <header>Thinking</header>
            <pre>{item.text}</pre>
          </article>
        {:else if item.role === "tool"}
          <article class="bubble tool">
            <header>Tool · {item.status}</header>
            <div class="tool-title">{item.title}</div>
            {#if item.detail}<div class="muted small">{item.detail}</div>{/if}
          </article>
        {:else}
          <article class="bubble system">
            <pre>{item.text}</pre>
          </article>
        {/if}
      {/each}
      {#if busy}
        <div class="typing muted">Agent working…</div>
      {/if}
    {/if}
  </main>

  <footer class="composer">
    <textarea
      bind:value={prompt}
      onkeydown={onKeydown}
      placeholder={connected
        ? "Message Grok Build… (Enter to send, Shift+Enter for newline)"
        : "Connect to a project first"}
      disabled={!connected || busy}
      rows="3"
    ></textarea>
    <button
      type="button"
      class="btn primary send"
      onclick={sendPrompt}
      disabled={!connected || busy || !prompt.trim()}
    >
      {busy ? "…" : "Send"}
    </button>
  </footer>
</div>

<style>
  :global(html, body) {
    margin: 0;
    height: 100%;
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
    height: 100vh;
    display: grid;
    grid-template-rows: auto auto auto 1fr auto;
    min-height: 0;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.85rem 1.1rem;
    border-bottom: 1px solid #1e2430;
    background: #12151a;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.7rem;
    min-width: 0;
  }

  .mark {
    font-size: 1.35rem;
    color: #7dd3fc;
  }

  h1 {
    margin: 0;
    font-size: 1.05rem;
    font-weight: 600;
    letter-spacing: -0.02em;
  }

  .tagline {
    margin: 0.15rem 0 0;
    font-size: 0.78rem;
    color: #8b93a7;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.35rem;
  }

  .pill {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 999px;
    padding: 0.05rem 0.45rem;
    font-size: 0.72rem;
    color: #a5b4fc;
  }

  .header-actions {
    display: flex;
    gap: 0.4rem;
  }

  .toolbar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
    padding: 0.65rem 1.1rem;
    border-bottom: 1px solid #1e2430;
    background: #10141b;
  }

  .cwd-label {
    font-size: 0.75rem;
    color: #8b93a7;
  }

  .cwd {
    flex: 1;
    min-width: 200px;
    background: #0d0f12;
    border: 1px solid #2a3344;
    border-radius: 8px;
    color: #e8eaed;
    padding: 0.45rem 0.65rem;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.8rem;
  }

  .cwd:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .banner {
    margin: 0;
    padding: 0.5rem 1.1rem;
    font-size: 0.85rem;
  }

  .banner.bad {
    background: #3f1d1d;
    color: #fca5a5;
    border-bottom: 1px solid #7f1d1d;
  }

  .logs {
    max-height: 120px;
    overflow: auto;
    padding: 0.4rem 1.1rem;
    background: #0a0c10;
    border-bottom: 1px solid #1e2430;
    font-family: ui-monospace, Menlo, monospace;
    font-size: 0.7rem;
    color: #94a3b8;
  }

  .log-line {
    white-space: pre-wrap;
    word-break: break-all;
  }

  .chat {
    overflow: auto;
    padding: 1rem 1.1rem 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 0.65rem;
    min-height: 0;
  }

  .empty {
    margin: auto;
    text-align: center;
    max-width: 28rem;
    color: #c5cad6;
    line-height: 1.5;
  }

  .bubble {
    max-width: min(720px, 100%);
    border-radius: 12px;
    padding: 0.65rem 0.85rem;
    border: 1px solid #232a38;
  }

  .bubble header {
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: #8b93a7;
    margin-bottom: 0.35rem;
  }

  .bubble pre {
    margin: 0;
    white-space: pre-wrap;
    word-break: break-word;
    font-family: inherit;
    font-size: 0.92rem;
    line-height: 1.45;
  }

  .user {
    align-self: flex-end;
    background: #1a2744;
    border-color: #2a3f66;
  }

  .assistant {
    align-self: flex-start;
    background: #151922;
  }

  .thought {
    align-self: flex-start;
    background: #14131c;
    border-color: #2e2a44;
    opacity: 0.92;
  }

  .thought pre {
    font-size: 0.82rem;
    color: #a8a3c4;
  }

  .tool {
    align-self: flex-start;
    background: #121a16;
    border-color: #1e3a2f;
    font-size: 0.85rem;
  }

  .tool-title {
    color: #6ee7b7;
    font-family: ui-monospace, Menlo, monospace;
    font-size: 0.82rem;
  }

  .system {
    align-self: center;
    background: transparent;
    border-style: dashed;
    color: #8b93a7;
    font-size: 0.8rem;
  }

  .typing {
    font-size: 0.8rem;
    padding-left: 0.25rem;
  }

  .composer {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 0.55rem;
    padding: 0.75rem 1.1rem 1rem;
    border-top: 1px solid #1e2430;
    background: #12151a;
  }

  textarea {
    resize: none;
    border-radius: 10px;
    border: 1px solid #2a3344;
    background: #0d0f12;
    color: #e8eaed;
    padding: 0.65rem 0.75rem;
    font: inherit;
    font-size: 0.92rem;
    line-height: 1.4;
  }

  textarea:focus {
    outline: none;
    border-color: #3b82f6;
  }

  textarea:disabled {
    opacity: 0.55;
  }

  .btn {
    border: 1px solid #2a3344;
    background: #1a2030;
    color: #e8eaed;
    border-radius: 8px;
    padding: 0.45rem 0.85rem;
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    white-space: nowrap;
  }

  .btn:hover:not(:disabled) {
    border-color: #3b82f6;
    background: #1e293b;
  }

  .btn:disabled {
    opacity: 0.55;
    cursor: default;
  }

  .btn.primary {
    background: #1d4ed8;
    border-color: #2563eb;
  }

  .btn.primary:hover:not(:disabled) {
    background: #2563eb;
  }

  .btn.danger {
    background: #3f1d1d;
    border-color: #7f1d1d;
  }

  .btn.ghost {
    background: transparent;
  }

  .btn.send {
    align-self: end;
    min-width: 4.5rem;
    height: 2.6rem;
  }

  .muted {
    color: #8b93a7;
  }

  .small {
    font-size: 0.75rem;
    margin-top: 0.2rem;
  }

  code {
    font-family: ui-monospace, Menlo, monospace;
    font-size: 0.85em;
    color: #a5b4fc;
  }
</style>

<script lang="ts">
  import { onDestroy, onMount, tick } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { fetchGrokStatus } from "$lib/grok";
  import { invoke } from "@tauri-apps/api/core";
  import {
    acpCancel,
    acpConnect,
    acpDisconnect,
    acpPrompt,
    acpRespondPermission,
    contentText,
    onAcpError,
    onAcpLog,
    onAcpPermission,
    onAcpStatus,
    onAcpUpdate,
  } from "$lib/acp";
  import { loadRecentSessions, saveRecentSession } from "$lib/sessions";
  import {
    getLastCwd,
    isOnboardingDone,
    setLastCwd,
    setOnboardingDone,
  } from "$lib/storage";
  import type { OrchestrationLoop } from "$lib/loops";
  import { buildSessionMarkdown, wrapUpPrompt } from "$lib/notes";
  import {
    accumulateUsage,
    emptyUsage,
    formatUsage,
    type UsageSnapshot,
  } from "$lib/usage";
  import type {
    ChatItem,
    GrokStatus,
    KickoffAnswers,
    KickoffPlan,
    LoopPhase,
    PermissionEvent,
    RecentSession,
  } from "$lib/types";
  import type { UnlistenFn } from "@tauri-apps/api/event";
  import HelpTip from "$lib/components/HelpTip.svelte";
  import GuidedSetup from "$lib/components/GuidedSetup.svelte";
  import GuidedKickoff from "$lib/components/GuidedKickoff.svelte";
  import PermissionModal from "$lib/components/PermissionModal.svelte";
  import OrchestrationPanel from "$lib/components/OrchestrationPanel.svelte";
  import Markdown from "$lib/components/Markdown.svelte";

  let status = $state<GrokStatus | null>(null);
  let statusLoading = $state(true);

  let cwd = $state(getLastCwd(""));
  let alwaysApprove = $state(false);
  let modelChoice = $state("");
  let effortChoice = $state("high");
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
  let recent = $state<RecentSession[]>([]);
  let usage = $state<UsageSnapshot>(emptyUsage());

  let showSetup = $state(false);
  let showKickoff = $state(false);
  let showOrch = $state(false);
  let exporting = $state(false);
  let lastExportPath = $state<string | null>(null);
  let permissionReq = $state<PermissionEvent | null>(null);

  let loopRunning = $state(false);
  let loopStop = $state(false);
  let loopStepIndex = $state(-1);
  let activeLoopId = $state<string | null>(null);
  let loopPhase = $state<LoopPhase>("idle");
  let lastLoopName = $state<string | null>(null);
  let lastLoopGoal = $state<string | null>(null);

  let scrollEl: HTMLElement | undefined = $state();
  let unlisteners: UnlistenFn[] = [];

  let idSeq = 0;
  function nid(prefix: string) {
    idSeq += 1;
    return `${prefix}-${idSeq}`;
  }

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

    if (kind === "user_message_chunk") return;

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
      const toolCallId =
        typeof update.toolCallId === "string"
          ? update.toolCallId
          : typeof update.tool_call_id === "string"
            ? update.tool_call_id
            : typeof (update as { id?: unknown }).id === "string"
              ? String((update as { id: string }).id)
              : undefined;
      const title =
        (typeof update.title === "string" && update.title) ||
        (typeof update.toolName === "string" && update.toolName) ||
        "tool";
      const toolStatus =
        (typeof update.status === "string" && update.status) || kind;
      const detail = typeof update.kind === "string" ? update.kind : undefined;

      if (toolCallId) {
        const existing = items.findIndex(
          (it) => it.role === "tool" && it.toolCallId === toolCallId,
        );
        if (existing >= 0) {
          items = items.map((it, i) =>
            i === existing && it.role === "tool"
              ? {
                  ...it,
                  title: title !== "tool" ? title : it.title,
                  status: toolStatus,
                  detail: detail ?? it.detail,
                }
              : it,
          );
          scrollToBottom();
          return;
        }
      }

      items = [
        ...items,
        {
          id: nid("tool"),
          role: "tool",
          title,
          status: toolStatus,
          detail,
          toolCallId,
        },
      ];
      scrollToBottom();
      return;
    }

    if (kind === "plan") {
      pushSystem("Plan update received");
    }
  }

  async function pickFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Choose project folder",
      });
      if (typeof selected === "string" && selected) {
        cwd = selected;
        setLastCwd(selected);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  async function doConnect(opts: {
    resumeSessionId?: string | null;
    rules?: string | null;
    alwaysApprove?: boolean;
    clearChat?: boolean;
    resetUsage?: boolean;
  } = {}) {
    error = null;
    connecting = true;
    streamAssistantId = null;
    streamThoughtId = null;
    const aa = opts.alwaysApprove ?? alwaysApprove;
    try {
      const res = await acpConnect({
        cwd: cwd.trim(),
        alwaysApprove: aa,
        resumeSessionId: opts.resumeSessionId ?? null,
        rules: opts.rules ?? null,
        model: modelChoice.trim() || null,
        effort: effortChoice.trim() || null,
      });
      connected = true;
      sessionId = res.sessionId;
      modelId = res.modelId;
      alwaysApprove = res.alwaysApprove;
      connectionMessage = res.resumed
        ? `Resumed ${res.sessionId.slice(0, 8)}…`
        : `Session ${res.sessionId.slice(0, 8)}…`;
      setLastCwd(res.cwd);
      saveRecentSession({
        sessionId: res.sessionId,
        cwd: res.cwd,
        modelId: res.modelId,
        updatedAt: Date.now(),
      });
      recent = loadRecentSessions();
      if (opts.clearChat !== false) {
        items = [];
      }
      if (opts.resetUsage !== false && !opts.resumeSessionId) {
        usage = emptyUsage();
      }
      pushSystem(
        `${res.resumed ? "Resumed" : "Connected"} · Grok Build${res.modelId ? ` · ${res.modelId}` : ""} · ${res.alwaysApprove ? "auto-approve" : "ask on tools"}${effortChoice ? ` · effort ${effortChoice}` : ""}`,
      );
      return res;
    } catch (e) {
      connected = false;
      sessionId = null;
      error = e instanceof Error ? e.message : String(e);
      return null;
    } finally {
      connecting = false;
    }
  }

  async function connect() {
    await doConnect();
  }

  async function resume(s: RecentSession) {
    cwd = s.cwd;
    setLastCwd(s.cwd);
    await doConnect({ resumeSessionId: s.sessionId });
  }

  async function launchKickoff(plan: KickoffPlan, _answers: KickoffAnswers) {
    showKickoff = false;
    alwaysApprove = plan.alwaysApprove;
    const res = await doConnect({
      rules: plan.rules,
      alwaysApprove: plan.alwaysApprove,
    });
    if (!res) return;
    pushSystem(`Kickoff plan: ${plan.summary}`);
    // Fire the crafted starter prompt
    prompt = plan.starterPrompt;
    await sendPrompt();
  }

  async function disconnect() {
    error = null;
    loopStop = true;
    loopRunning = false;
    loopPhase = "idle";
    activeLoopId = null;
    loopStepIndex = -1;
    try {
      if (busy) await acpCancel().catch(() => {});
      await acpDisconnect();
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      connected = false;
      sessionId = null;
      modelId = null;
      connectionMessage = "";
      busy = false;
      permissionReq = null;
      pushSystem("Disconnected");
    }
  }

  /** Core prompt send. Returns false if cancelled/failed. */
  async function runPromptTurn(text: string, opts: { showInComposer?: boolean } = {}) {
    if (!text.trim() || !connected) return false;
    if (busy) return false;

    error = null;
    busy = true;
    streamAssistantId = null;
    streamThoughtId = null;
    items = [...items, { id: nid("user"), role: "user", text }];
    if (opts.showInComposer !== false) {
      /* no-op: caller clears composer when needed */
    }
    await scrollToBottom();

    try {
      const result = await acpPrompt(text);
      streamAssistantId = null;
      streamThoughtId = null;
      usage = accumulateUsage(usage, result.meta);
      if (result.stopReason) {
        pushSystem(`Turn complete · ${result.stopReason}`);
      }
      if (sessionId) {
        saveRecentSession({
          sessionId,
          cwd: cwd.trim(),
          modelId,
          updatedAt: Date.now(),
        });
        recent = loadRecentSessions();
      }
      return true;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      if (msg.toLowerCase().includes("cancel")) {
        pushSystem("Turn cancelled");
      } else {
        error = msg;
        pushSystem(`Error: ${msg}`);
      }
      return false;
    } finally {
      busy = false;
      streamAssistantId = null;
      streamThoughtId = null;
    }
  }

  async function sendPrompt() {
    const text = prompt.trim();
    if (!text || !connected || busy || loopRunning) return;
    prompt = "";
    await runPromptTurn(text);
  }

  async function cancelTurn() {
    try {
      await acpCancel();
      if (loopRunning) loopStop = true;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  async function startLoop(loop: OrchestrationLoop, goal: string) {
    loopStop = false;
    showOrch = true;
    activeLoopId = loop.id;
    lastLoopName = loop.name;
    lastLoopGoal = goal;
    loopRunning = true;
    loopPhase = "running";
    loopStepIndex = 0;

    if (!connected) {
      alwaysApprove = loop.preferAutoApprove || alwaysApprove;
      const res = await doConnect({
        rules: loop.rules,
        alwaysApprove: loop.preferAutoApprove ? true : alwaysApprove,
      });
      if (!res) {
        loopRunning = false;
        loopPhase = "idle";
        activeLoopId = null;
        loopStepIndex = -1;
        return;
      }
    }

    items = [
      ...items,
      {
        id: nid("loop"),
        role: "loop",
        text: `▶ Starting loop: ${loop.name}\nGoal: ${goal}\nSteps: ${loop.steps.map((s) => s.label).join(" → ")}`,
      },
    ];

    const completed: string[] = [];
    let finishedAll = false;
    for (let i = 0; i < loop.steps.length; i++) {
      if (loopStop) break;
      loopStepIndex = i;
      const step = loop.steps[i];
      pushSystem(`Loop · step ${i + 1} of ${loop.steps.length}: ${step.label}`);
      const stepPrompt = step.buildPrompt(goal, completed);
      const ok = await runPromptTurn(stepPrompt);
      if (!ok || loopStop) break;
      completed.push(step.label);
    }

    finishedAll = !loopStop && completed.length === loop.steps.length;

    if (finishedAll && !loopStop) {
      pushSystem(`Loop · wrap-up: summarizing what shipped`);
      loopStepIndex = loop.steps.length;
      await runPromptTurn(wrapUpPrompt(loop.name, goal, completed));
    }

    loopRunning = false;

    if (finishedAll && !loopStop) {
      loopPhase = "complete";
      loopStepIndex = loop.steps.length + 1;
      items = [
        ...items,
        {
          id: nid("loop"),
          role: "loop",
          text: `✓ LOOP FINISHED — ${loop.name}\nGoal: ${goal}\nCompleted: ${completed.join(" → ")} → Wrap-up\n\nTip: click Export notes to save a markdown journal in your project (.grok-desktop/notes/).`,
        },
      ];
      await scrollToBottom();
      showOrch = true;
    } else {
      loopPhase = "stopped";
      items = [
        ...items,
        {
          id: nid("loop"),
          role: "loop",
          text: `■ Loop stopped — ${loop.name}\nFinished steps: ${completed.length ? completed.join(" → ") : "none"}`,
        },
      ];
      await scrollToBottom();
      showOrch = true;
    }
  }

  async function exportNotes() {
    if (!cwd.trim() || items.length === 0) {
      error = "Nothing to export yet — run a chat or loop first.";
      return;
    }
    exporting = true;
    error = null;
    try {
      const markdown = buildSessionMarkdown({
        cwd: cwd.trim(),
        sessionId,
        modelId,
        usage,
        items,
        loopName: lastLoopName,
        loopGoal: lastLoopGoal,
      });
      const stamp = new Date().toISOString().slice(0, 19).replace(/[:T]/g, "-");
      const res = await invoke<{ path: string }>("export_session_notes", {
        cwd: cwd.trim(),
        markdown,
        suggestedName: lastLoopName
          ? `loop-${lastLoopName.toLowerCase().replace(/\s+/g, "-")}-${stamp}`
          : `session-${stamp}`,
      });
      lastExportPath = res.path;
      pushSystem(`Notes exported → ${res.path}`);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      exporting = false;
    }
  }

  function stopLoop() {
    loopStop = true;
    cancelTurn();
  }

  function dismissLoopResult() {
    loopPhase = "idle";
    activeLoopId = null;
    loopStepIndex = -1;
    showOrch = false;
  }

  async function replyPermission(optionId: string | null, cancelled = false) {
    if (!permissionReq) return;
    const requestId = permissionReq.requestId;
    permissionReq = null;
    try {
      await acpRespondPermission({
        requestId,
        outcome: cancelled || !optionId ? "cancelled" : "selected",
        optionId: optionId,
      });
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      sendPrompt();
    }
  }

  function finishSetup() {
    setOnboardingDone();
    showSetup = false;
  }

  onMount(async () => {
    recent = loadRecentSessions();
    showSetup = !isOnboardingDone();
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
      onAcpPermission((ev) => {
        permissionReq = ev;
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

{#if showSetup}
  <GuidedSetup
    status={status}
    statusLoading={statusLoading}
    cwd={cwd}
    onRecheck={refreshStatus}
    onPickFolder={pickFolder}
    onDone={finishSetup}
    onKickoff={() => (showKickoff = true)}
    onLoops={() => (showOrch = true)}
    onConnect={() => connect()}
  />
{/if}

<GuidedKickoff
  open={showKickoff}
  onClose={() => (showKickoff = false)}
  onLaunch={launchKickoff}
/>

<PermissionModal request={permissionReq} onReply={replyPermission} />

<OrchestrationPanel
  open={showOrch}
  connected={connected}
  busy={busy}
  loopRunning={loopRunning}
  loopPhase={loopPhase}
  currentStepIndex={loopStepIndex}
  activeLoopId={activeLoopId}
  lastLoopName={lastLoopName}
  lastLoopGoal={lastLoopGoal}
  onClose={() => {
    if (loopPhase === "running") return;
    if (loopPhase === "complete" || loopPhase === "stopped") dismissLoopResult();
    else showOrch = false;
  }}
  onStart={startLoop}
  onStop={stopLoop}
  onDismissResult={dismissLoopResult}
/>

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
            {#if usage.turns > 0}<span class="pill muted-pill">{formatUsage(usage)}</span>{/if}
          {:else if status?.ready}
            CLI ready — kickoff, loops, or connect
          {:else}
            {status?.message ?? "Checking Grok CLI…"}
          {/if}
        </p>
      </div>
    </div>
    <div class="header-actions">
      <span
        class="cli-chip"
        class:ok={status?.ready}
        class:bad={status && !status.ready}
        class:loading={statusLoading}
        title={status?.message ?? "Checking Grok Build CLI…"}
      >
        <span class="cli-dot"></span>
        {#if statusLoading && !status}
          CLI…
        {:else if status?.ready}
          CLI ready
        {:else}
          CLI issue
        {/if}
      </span>
      <HelpTip title="How this works" label="?">
        <p>
          Grok Desktop is a thin shell over the official <strong>Grok Build</strong> CLI via ACP.
        </p>
        <ul>
          <li><strong>Guided kickoff</strong> — interview → strong first prompt.</li>
          <li><strong>Loops</strong> — multi-step explore/plan/implement sequences.</li>
          <li><strong>Connect</strong> — free-form chat on a project folder.</li>
        </ul>
      </HelpTip>
      <button type="button" class="btn ghost" onclick={() => (showSetup = true)}>Setup</button>
      <button
        type="button"
        class="btn ghost"
        onclick={exportNotes}
        disabled={exporting || items.length === 0}
        title="Save session notes as markdown in the project"
      >
        {exporting ? "…" : "Export"}
      </button>
      <button
        type="button"
        class="btn ghost"
        onclick={refreshStatus}
        disabled={statusLoading}
        title="Re-check Grok CLI install + auth"
      >
        {statusLoading ? "…" : "Recheck"}
      </button>
      <button type="button" class="btn ghost" onclick={() => (showLogs = !showLogs)}>Logs</button>
    </div>
  </header>

  <section class="toolbar">
    <div class="cwd-row">
      <label class="cwd-label" for="cwd">Project</label>
      <input
        id="cwd"
        class="cwd"
        bind:value={cwd}
        disabled={connected || connecting}
        placeholder="/absolute/path/to/project"
        spellcheck="false"
        onchange={() => setLastCwd(cwd)}
      />
      <button
        type="button"
        class="btn"
        onclick={pickFolder}
        disabled={connected || connecting}
      >
        Browse…
      </button>
    </div>
    <div class="toolbar-actions">
      <label class="field">
        <span>Effort</span>
        <select bind:value={effortChoice} disabled={connected || connecting}>
          <option value="low">low</option>
          <option value="medium">medium</option>
          <option value="high">high</option>
          <option value="xhigh">xhigh</option>
        </select>
      </label>
      <label class="field model-field">
        <span>Model</span>
        <input
          bind:value={modelChoice}
          disabled={connected || connecting}
          placeholder="default"
          spellcheck="false"
        />
      </label>
      <label class="check" class:disabled={connected}>
        <input type="checkbox" bind:checked={alwaysApprove} disabled={connected || connecting} />
        Auto-approve
        <HelpTip title="Auto-approve" label="?">
          <p>
            When on, tools run without asking (faster vibe coding). When off, you’ll get a
            permission popup if the agent requests one.
          </p>
        </HelpTip>
      </label>
      <button
        type="button"
        class="btn accent"
        onclick={() => (showOrch = true)}
        disabled={connecting || !status?.ready || (busy && !loopRunning)}
      >
        Loops
      </button>
      {#if !connected}
        <button
          type="button"
          class="btn accent"
          onclick={() => (showKickoff = true)}
          disabled={connecting || !status?.ready}
        >
          Kickoff
        </button>
        <button
          type="button"
          class="btn primary"
          onclick={connect}
          disabled={connecting || !status?.ready}
        >
          {connecting ? "Connecting…" : "Connect"}
        </button>
      {:else}
        <button type="button" class="btn danger" onclick={disconnect}>
          Disconnect
        </button>
      {/if}
    </div>
  </section>

  {#if !connected && recent.length > 0}
    <section class="recent" aria-label="Recent sessions">
      <span class="recent-label">Resume</span>
      {#each recent.slice(0, 5) as s}
        <button
          type="button"
          class="recent-chip"
          disabled={connecting || !status?.ready}
          onclick={() => resume(s)}
          title={s.cwd}
        >
          {s.sessionId.slice(0, 8)}… · {s.cwd.split("/").filter(Boolean).slice(-2).join("/")}
        </button>
      {/each}
    </section>
  {/if}

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
        <h2>Build with Grok — without fighting the terminal</h2>
        <p>
          New here? Use <strong>Kickoff</strong> (interview) or <strong>Loops</strong> (explore →
          plan → implement → verify).
        </p>
        <p class="muted">
          Or <strong>Browse…</strong> → <strong>Connect</strong> → free-form chat.
        </p>
        <div class="empty-actions">
          <button
            type="button"
            class="btn accent"
            onclick={() => (showKickoff = true)}
            disabled={!status?.ready}
          >
            Guided kickoff
          </button>
          <button
            type="button"
            class="btn primary"
            onclick={() => (showOrch = true)}
            disabled={!status?.ready}
          >
            Run a loop
          </button>
        </div>
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
            <Markdown source={item.text} />
          </article>
        {:else if item.role === "thought"}
          <details class="bubble thought">
            <summary>Thinking</summary>
            <pre>{item.text}</pre>
          </details>
        {:else if item.role === "tool"}
          <article class="bubble tool">
            <header>Tool · {item.status}</header>
            <div class="tool-title">{item.title}</div>
            {#if item.detail}<div class="muted small">{item.detail}</div>{/if}
          </article>
        {:else if item.role === "loop"}
          <article class="bubble loop">
            <pre>{item.text}</pre>
          </article>
        {:else}
          <article class="bubble system">
            <pre>{item.text}</pre>
          </article>
        {/if}
      {/each}
      {#if busy}
        <div class="typing muted">
          {#if loopRunning}
            Loop step {(loopStepIndex >= 0 ? loopStepIndex : 0) + 1}… agent working
          {:else}
            Agent working…
          {/if}
        </div>
      {/if}
    {/if}
  </main>

  <footer class="composer">
    <textarea
      bind:value={prompt}
      onkeydown={onKeydown}
      placeholder={connected
        ? loopRunning
          ? "Loop running — wait or Cancel…"
          : "Message Grok Build… (Enter send · Shift+Enter newline)"
        : "Connect, Kickoff, or Loops first"}
      disabled={!connected || busy || loopRunning}
      rows="3"
    ></textarea>
    {#if busy}
      <button type="button" class="btn danger send" onclick={cancelTurn}>Cancel</button>
    {:else}
      <button
        type="button"
        class="btn primary send"
        onclick={sendPrompt}
        disabled={!connected || !prompt.trim() || loopRunning}
      >
        Send
      </button>
    {/if}
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
    grid-template-rows: auto auto auto auto 1fr auto;
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

  .muted-pill {
    color: #94a3b8;
    border-color: #2a3344;
  }

  .cli-chip {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.72rem;
    font-weight: 600;
    padding: 0.28rem 0.55rem;
    border-radius: 999px;
    border: 1px solid #2a3344;
    background: #151922;
    color: #8b93a7;
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }

  .cli-chip.ok {
    color: #86efac;
    border-color: #166534;
    background: #0f2a1f;
  }

  .cli-chip.bad {
    color: #fca5a5;
    border-color: #7f1d1d;
    background: #3f1d1d;
  }

  .cli-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #6b7280;
  }

  .cli-chip.ok .cli-dot {
    background: #34d399;
    box-shadow: 0 0 6px rgba(52, 211, 153, 0.6);
  }

  .cli-chip.bad .cli-dot {
    background: #f87171;
  }

  .cli-chip.loading {
    opacity: 0.75;
  }

  .field {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
    font-size: 0.75rem;
    color: #8b93a7;
  }

  .field select,
  .field input {
    background: #0d0f12;
    border: 1px solid #2a3344;
    border-radius: 6px;
    color: #e8eaed;
    padding: 0.3rem 0.4rem;
    font-size: 0.78rem;
  }

  .model-field input {
    width: 7.5rem;
    font-family: ui-monospace, Menlo, monospace;
  }

  .empty-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    justify-content: center;
    margin-top: 0.75rem;
  }

  .loop {
    align-self: center;
    background: #13221f;
    border-color: #1e4d3f;
    color: #6ee7b7;
    font-size: 0.85rem;
    max-width: min(640px, 100%);
    white-space: pre-wrap;
  }

  .loop pre {
    white-space: pre-wrap;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 0.4rem;
  }

  .toolbar {
    display: flex;
    flex-direction: column;
    gap: 0.55rem;
    padding: 0.65rem 1.1rem;
    border-bottom: 1px solid #1e2430;
    background: #10141b;
  }

  .cwd-row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .cwd-label {
    font-size: 0.75rem;
    color: #8b93a7;
  }

  .cwd {
    flex: 1;
    min-width: 180px;
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

  .toolbar-actions {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
    justify-content: flex-end;
  }

  .check {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.8rem;
    color: #c5cad6;
    margin-right: auto;
  }

  .check.disabled {
    opacity: 0.6;
  }

  .recent {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.4rem;
    padding: 0.4rem 1.1rem;
    border-bottom: 1px solid #1e2430;
    background: #0e1218;
  }

  .recent-label {
    font-size: 0.72rem;
    color: #8b93a7;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .recent-chip {
    border: 1px solid #2a3344;
    background: #151922;
    color: #c5cad6;
    border-radius: 999px;
    padding: 0.25rem 0.6rem;
    font-size: 0.72rem;
    cursor: pointer;
    font-family: ui-monospace, Menlo, monospace;
  }

  .recent-chip:hover:not(:disabled) {
    border-color: #3b82f6;
  }

  .recent-chip:disabled {
    opacity: 0.5;
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
    max-width: 32rem;
    color: #c5cad6;
    line-height: 1.5;
  }

  .empty h2 {
    margin: 0 0 0.6rem;
    font-size: 1.15rem;
    color: #e8eaed;
  }

  .empty .btn {
    margin-top: 0.75rem;
  }

  .bubble {
    max-width: min(720px, 100%);
    border-radius: 12px;
    padding: 0.65rem 0.85rem;
    border: 1px solid #232a38;
  }

  .bubble header,
  .thought summary {
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: #8b93a7;
    margin-bottom: 0.35rem;
  }

  .thought summary {
    cursor: pointer;
    list-style: none;
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
    opacity: 0.95;
  }

  .thought pre {
    font-size: 0.82rem;
    color: #a8a3c4;
    margin-top: 0.35rem;
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

  .btn.accent {
    background: #0f766e;
    border-color: #14b8a6;
  }

  .btn.accent:hover:not(:disabled) {
    background: #0d9488;
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

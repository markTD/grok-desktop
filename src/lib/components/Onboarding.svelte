<script lang="ts">
  let { onDone }: { onDone: () => void } = $props();

  let step = $state(0);
  const steps = [
    {
      title: "Welcome to Grok Desktop",
      body: "A friendly window on top of xAI’s Grok Build coding agent. The agent, tools, and login stay in the official grok CLI — this app is the easy UI.",
    },
    {
      title: "What you need",
      body: "1) Grok Build installed (x.ai/cli)\n2) Signed in (SuperGrok / supported plan)\n3) A project folder on your Mac\n\nIf the header says Ready + Authenticated, you’re good.",
    },
    {
      title: "Two ways to start",
      body: "• Guided kickoff — short interview, we write a strong first prompt and safety settings for you.\n• Connect — pick a folder and chat freely (like Claude Desktop).\n\nYou can resume recent sessions later.",
    },
    {
      title: "Safety",
      body: "By default we do not force auto-approve. For vibe-coding speed you can enable auto-approve in Guided kickoff (Fast) or the Connect bar.\n\nAlways use a git repo or backup when letting tools edit files.",
    },
  ];

  function next() {
    if (step < steps.length - 1) step += 1;
    else onDone();
  }
</script>

<div class="overlay" role="dialog" aria-modal="true" aria-labelledby="ob-title">
  <div class="card">
    <p class="step-label">Intro {step + 1} / {steps.length}</p>
    <h2 id="ob-title">{steps[step].title}</h2>
    <pre class="body">{steps[step].body}</pre>
    <div class="actions">
      <button type="button" class="btn ghost" onclick={onDone}>Skip</button>
      <button type="button" class="btn primary" onclick={next}>
        {step === steps.length - 1 ? "Start" : "Next"}
      </button>
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
    width: min(440px, 100%);
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
    font-size: 1.2rem;
  }

  .body {
    margin: 0;
    white-space: pre-wrap;
    font-family: inherit;
    font-size: 0.92rem;
    line-height: 1.5;
    color: #c5cad6;
  }

  .actions {
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

  .btn.primary {
    background: #1d4ed8;
    border-color: #2563eb;
  }

  .btn.ghost {
    background: transparent;
  }
</style>

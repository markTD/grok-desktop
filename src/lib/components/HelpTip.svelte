<script lang="ts">
  let {
    label = "Help",
    title = "Tip",
    children,
  }: {
    label?: string;
    title?: string;
    children?: import("svelte").Snippet;
  } = $props();

  let open = $state(false);
</script>

<span class="wrap">
  <button
    type="button"
    class="tip-btn"
    aria-expanded={open}
    aria-label={title}
    onclick={() => (open = !open)}
  >
    {label}
  </button>
  {#if open}
    <button type="button" class="backdrop" aria-label="Close tip" onclick={() => (open = false)}
    ></button>
    <div class="popover" role="dialog" aria-label={title}>
      <header>
        <strong>{title}</strong>
        <button type="button" class="x" onclick={() => (open = false)}>×</button>
      </header>
      <div class="body">
        {#if children}{@render children()}{/if}
      </div>
    </div>
  {/if}
</span>

<style>
  .wrap {
    position: relative;
    display: inline-flex;
  }

  .tip-btn {
    border: 1px solid #2a3344;
    background: #151922;
    color: #93c5fd;
    border-radius: 999px;
    width: 1.35rem;
    height: 1.35rem;
    font-size: 0.72rem;
    font-weight: 700;
    cursor: pointer;
    line-height: 1;
    padding: 0;
  }

  .tip-btn:hover {
    border-color: #3b82f6;
  }

  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 40;
    border: none;
    padding: 0;
    margin: 0;
    background: transparent;
    cursor: default;
  }

  .popover {
    position: absolute;
    top: calc(100% + 0.4rem);
    right: 0;
    z-index: 50;
    width: min(320px, 80vw);
    background: #151922;
    border: 1px solid #2a3344;
    border-radius: 10px;
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.45);
    padding: 0.65rem 0.75rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.35rem;
    font-size: 0.85rem;
  }

  .x {
    border: none;
    background: transparent;
    color: #8b93a7;
    font-size: 1.1rem;
    cursor: pointer;
  }

  .body {
    font-size: 0.8rem;
    color: #c5cad6;
    line-height: 1.45;
  }

  .body :global(p) {
    margin: 0 0 0.45rem;
  }

  .body :global(ul) {
    margin: 0;
    padding-left: 1.1rem;
  }
</style>

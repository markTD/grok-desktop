<script lang="ts">
  import { renderMarkdown } from "$lib/markdown";

  let { source }: { source: string } = $props();
  let html = $derived(renderMarkdown(source));
</script>

<div class="md">
  <!-- eslint-disable-next-line svelte/no-at-html-tags -->
  {@html html}
</div>

<style>
  .md :global(p) {
    margin: 0 0 0.55rem;
    line-height: 1.5;
  }
  .md :global(p:last-child) {
    margin-bottom: 0;
  }
  .md :global(code) {
    font-family: ui-monospace, Menlo, monospace;
    font-size: 0.85em;
    background: #0d0f12;
    padding: 0.1em 0.3em;
    border-radius: 4px;
  }
  .md :global(pre) {
    background: #0d0f12;
    padding: 0.65rem 0.75rem;
    border-radius: 8px;
    overflow: auto;
    font-size: 0.82rem;
  }
  .md :global(pre code) {
    background: none;
    padding: 0;
  }
  .md :global(ul),
  .md :global(ol) {
    margin: 0.35rem 0;
    padding-left: 1.25rem;
  }
  .md :global(a) {
    color: #7dd3fc;
  }
  .md :global(strong) {
    font-weight: 600;
  }
</style>

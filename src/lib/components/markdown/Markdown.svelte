<script lang="ts">
  import { marked } from "marked";
  import DOMPurify from "isomorphic-dompurify";
  import "$lib/styles/github-markdown.css";

  type Props = {
    source: string;
  };

  const renderer = new marked.Renderer();

  DOMPurify.addHook("afterSanitizeAttributes", (currentNode: unknown) => {
    if (currentNode instanceof HTMLAnchorElement) {
      currentNode.setAttribute("target", "_blank");
    }
  });

  const { source }: Props = $props();
  const markdownPromise = $derived(
    marked(source, {
      renderer,
    }),
  );
</script>

{#key source}
  <div class="markdown-body">
    {#await markdownPromise then rendered}
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html DOMPurify.sanitize(rendered)}
    {/await}
  </div>
{/key}

<style>
  .markdown-body {
    width: 100%;
    height: 100%;
    padding: 1rem;
    padding-bottom: 10rem;
    overflow: auto;
  }
</style>

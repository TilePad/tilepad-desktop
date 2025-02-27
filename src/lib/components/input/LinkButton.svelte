<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAnchorAttributes } from "svelte/elements";

  type Props = {
    variant?: "default" | "warning" | "error";
    children?: Snippet;
  } & HTMLAnchorAttributes;

  const { variant, children, ...props }: Props = $props();
</script>

<a class="btn" {...props} data-variant={variant}>
  {@render children?.()}
</a>

<style>
  .btn {
    padding: 0.5rem;
    background-color: #544d5e;
    border: none;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;
  }

  .btn[data-variant="warning"] {
    background-color: #423f1b;
    color: #f0ee82;
  }

  .btn[data-variant="error"] {
    background-color: #795151;
    color: hsl(0, 100%, 88%);
  }

  .btn:hover {
    background-color: #675d75;
  }

  .btn[data-variant="warning"]:hover {
    background-color: #775538;
    color: #f5d6a7;
  }

  .btn:disabled,
  .btn:disabled:hover {
    background-color: #222;
    border: 1px solid #333;
    cursor: not-allowed;
    color: #777;
  }
</style>

<script module lang="ts">
  export type ButtonProps = {
    variant?: "default" | "warning" | "error";
    children?: Snippet;
  } & HTMLButtonAttributes;
</script>

<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  const { variant = "default", children, ...props }: ButtonProps = $props();
</script>

<button
  type="button"
  {...props}
  class="btn {props.class}"
  data-variant={variant}
>
  {@render children?.()}
</button>

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
    color: #ffc2c2;
  }

  .btn:hover {
    background-color: #675d75;
  }

  .btn[data-variant="warning"]:hover {
    background-color: #775538;
    color: #f5d6a7;
  }

  .btn[data-variant="error"]:hover {
    background-color: #966464;
    color: rgb(228, 196, 196);
  }

  .btn:disabled,
  .btn:disabled:hover {
    background-color: #222;
    border: 1px solid #333;
    cursor: not-allowed;
    color: #777;
  }
</style>

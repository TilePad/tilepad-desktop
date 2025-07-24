<script module lang="ts">
  export type ButtonProps = {
    variant?: "default" | "warning" | "error";
    transparent?: boolean;
    children?: Snippet;
    size?: ButtonSize;
    loading?: boolean;
  } & HTMLButtonAttributes;

  export type ButtonSize = "default" | "small";
</script>

<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  import LoadingSpinner from "../loading/LoadingSpinner.svelte";

  const {
    variant = "default",
    transparent,
    size = "default",
    children,
    loading,
    disabled,
    ...props
  }: ButtonProps = $props();
</script>

<button
  type="button"
  {...props}
  disabled={disabled || loading}
  class="btn {props.class}"
  class:btn--transparent={transparent}
  class:btn--loading={loading}
  data-size={size}
  data-variant={variant}
>
  {#if loading}
    <LoadingSpinner size={size === "small" ? 15 : 20} />
  {/if}

  {@render children?.()}
</button>

<style>
  .btn {
    display: flex;
    align-items: center;

    background-color: #544d5e;
    border: none;
    color: #fff;
    border-radius: var(--tp-radius-base);

    cursor: pointer;
    text-decoration: none;
    position: relative;

    font-size: var(--tp-text-base);
    gap: var(--tp-space-2);
    padding: 0 var(--tp-btn-padding-x-md);
    height: var(--tp-btn-height-md);
  }

  .btn[data-size="small"] {
    padding: 0 var(--tp-btn-padding-x-sm);
    height: var(--tp-btn-height-sm);
    font-size: var(--tp-text-sm);
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
    background-color: #3e3946;
    border: 1px solid #333;
    cursor: not-allowed;
    color: #b3b3b3;
  }

  .btn--transparent {
    background-color: transparent !important;
    color: #ffffff;
    border-color: transparent !important;
  }
</style>

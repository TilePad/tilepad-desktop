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
    <span class="loading-spinner"></span>
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

  .btn--loading {
    position: relative;
    padding-left: 40px !important;
  }

  .btn--loading.btn[data-size="small"] {
    padding-left: 35px !important;
  }

  .btn--loading.btn[data-size="small"] .loading-spinner {
    left: 10px;
    width: 15px;
    height: 15px;
  }

  .btn--loading .loading-spinner {
    display: inline-block;
  }

  .loading-spinner {
    display: none;
    width: 20px;
    height: 20px;
    border: 2px solid #777;
    border-radius: 50%;
    border-top-color: white;
    animation: spin 1s ease-in-out infinite;
    position: absolute;
    left: 10px;
    top: 50%;
    transform: translateY(-50%);
  }

  @keyframes spin {
    to {
      transform: translateY(-50%) rotate(360deg);
    }
  }
</style>

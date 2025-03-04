<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  type Props = {
    children?: Snippet;
    active: boolean;
  } & HTMLButtonAttributes;

  const { active, children, ...props }: Props = $props();
</script>

<button
  type="button"
  class:btn--active={active}
  {...props}
  class="btn {props.class}"
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
    display: inline-flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;
  }

  .btn--active {
    background-color: #8971aa;
  }

  .btn:hover {
    background-color: #675d75;
  }

  .btn--active:hover {
    background-color: #a58ec5;
  }

  .btn:disabled,
  .btn:disabled:hover {
    background-color: #222;
    border: 1px solid #333;
    cursor: not-allowed;
    color: #777;
  }
</style>

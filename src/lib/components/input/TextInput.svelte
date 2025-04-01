<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  type Props = {
    type?: "text" | "password";
    fullWidth?: boolean;
  } & Omit<HTMLInputAttributes, "type">;

  let {
    value = $bindable(),
    type = "text",
    fullWidth,
    ...props
  }: Props = $props();
</script>

<input bind:value {...props} {type} class:input--full-width={fullWidth} />

<style>
  input {
    padding: 0.5rem;
    background-color: #1f1d22;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
  }

  .input--full-width {
    width: 100%;
  }
</style>

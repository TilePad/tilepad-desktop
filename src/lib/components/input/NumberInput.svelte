<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  type Props = {
    type?: "number";
  } & Omit<HTMLInputAttributes, "type">;

  const { type = "number", ...props }: Props = $props();
</script>

<input {...props} {type} />

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
</style>

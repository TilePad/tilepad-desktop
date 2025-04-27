<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";

  type Props = HTMLAttributes<HTMLDivElement> & {};

  const props: Props = $props();
</script>

<div {...props} class="skeleton-list">
  <div class="skeleton" style="width: 80%; height: 1rem"></div>
  <div class="skeleton" style="width: 70%; height: 1rem"></div>
  <div class="skeleton" style="width: 30%; height: 1rem"></div>
</div>

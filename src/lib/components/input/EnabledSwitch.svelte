<script lang="ts">
  import { Switch, type SwitchRootProps } from "bits-ui";

  type Props = {
    checked: boolean;
    onCheckedChange: (value: boolean) => void;
  } & SwitchRootProps;

  const { checked, onCheckedChange, ...props }: Props = $props();
</script>

<Switch.Root {...props} {checked} {onCheckedChange}>
  {#snippet child({ checked, props })}
    <button {...props} class="switch" data-checked={checked}>
      <span class="switch-thumb"></span>
    </button>
  {/snippet}
</Switch.Root>

<style>
  .indicator {
    color: #fff;
  }

  .switch {
    display: block;
    width: 48px;
    height: 26px;
    align-items: center;
    justify-content: center;
    border-radius: 0.375rem;
    border: 1px solid #999;
    background-color: #333;
    transition: all 150ms ease-in-out;
    cursor: pointer;
    flex-shrink: 0;

    background-color: #702828;
  }

  .switch-thumb {
    width: 24px;
    height: 24px;
    border-radius: 0.3rem;
    background-color: #ccc;
    display: block;
  }

  .switch[data-checked="true"] {
    background-color: #337933;
    border: 1px solid #999;
  }

  .switch[data-checked="true"] .switch-thumb {
    transform: translateX(24px);
  }
</style>

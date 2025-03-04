<script lang="ts">
  import type { TileId, TileLabel, TileConfig } from "$lib/api/types/tiles";

  import { watch, useDebounce } from "runed";
  import { updateTile } from "$lib/api/tiles";

  import TextInput from "../input/TextInput.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";

  type Props = {
    tileId: TileId;
    config: TileConfig;
  };

  const { tileId, config }: Props = $props();

  let label = $state(config.label);
  let dirty = $state(false);

  const updateLabel = useDebounce((label: TileLabel) => {
    updateTile(tileId, {
      config: {
        ...config,
        label,
      },
    });
  }, 150);

  const onChangeTileName = (event: Event) => {
    const value = (event.target as HTMLInputElement).value;
    dirty = true;
    label = { ...label, label: value };
    updateLabel(label);
  };

  // Only set initial site when not dirty (Prevent UI flickering from changes)
  watch(
    () => ({
      config,
      dirty,
    }),
    ({ config, dirty }) => {
      if (dirty) return;
      label = config.label;
    },
  );
</script>

<TextInput value={label.label} oninput={onChangeTileName} />

<PopoverButton>
  {#snippet children()}T{/snippet}
  {#snippet content()}{/snippet}
</PopoverButton>

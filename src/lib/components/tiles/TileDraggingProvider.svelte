<script lang="ts" module>
  import type { ActionId } from "$lib/api/types/actions";

  import { type TileId } from "$lib/api/types/tiles";

  const tileDraggingContextKey = Symbol("TILE_DRAGGING_CONTEXT");

  type DraggingData =
    | { type: "tile"; tileId: TileId }
    | { type: "action"; pluginId: PluginId; actionId: ActionId };

  type DraggingState = {
    // Current dragged element
    previewElement: HTMLElement;
    // Drag start position
    initialX: number;
    initialY: number;
    // Data
    data: DraggingData;
  };

  type DropZoneTarget =
    | {
        type: "emptyTile";
        row: number;
        column: number;
      }
    | {
        type: "filledTile";
        row: number;
        column: number;
      }
    | { type: "sidebar" };

  type TileDraggingContext = {
    draggingState(): DraggingState | null;
    dropZoneTarget(): DropZoneTarget | null;

    onStartDragging: (
      event: PointerEvent,
      data: DraggingData,
      previewElement: HTMLElement,
    ) => void;
  };

  export function getDraggingContext(): TileDraggingContext {
    return getContext(tileDraggingContextKey);
  }
</script>

<script lang="ts">
  import type { PluginId } from "$lib/api/types/plugin";

  import { getContext, setContext, type Snippet } from "svelte";

  type Props = {
    onMoveTile: (tileId: TileId, row: number, column: number) => void;
    onDeleteTile: (tileId: TileId) => void;
    onPlaceTile: (
      pluginId: PluginId,
      actionId: ActionId,
      row: number,
      column: number,
    ) => void;

    children?: Snippet;
  };

  const { onMoveTile, onDeleteTile, onPlaceTile, children }: Props = $props();

  let dropZoneTarget: DropZoneTarget | null = $state(null);
  let draggingState: DraggingState | null = $state(null);

  setContext<TileDraggingContext>(tileDraggingContextKey, {
    onStartDragging,

    dropZoneTarget() {
      return dropZoneTarget;
    },
    draggingState() {
      return draggingState;
    },
  });

  function onStartDragging(
    event: PointerEvent,
    data: DraggingData,
    dragElement: HTMLElement,
  ) {
    // Prevent text selection
    event.preventDefault();

    const previewRect = dragElement.getBoundingClientRect();
    const computedStyles = getComputedStyle(dragElement);
    const previewElement = dragElement.cloneNode(true) as HTMLButtonElement;

    dropZoneTarget = null;
    draggingState = {
      previewElement: previewElement,
      initialX: event.clientX - previewRect.left,
      initialY: event.clientY - previewRect.top,
      data,
    };

    // Setup initial preview styles
    previewElement.style.position = "absolute";
    previewElement.style.cursor = "grabbing";
    previewElement.style.left = `${event.clientX - draggingState.initialX}px`;
    previewElement.style.top = `${event.clientY - draggingState.initialY}px`;
    previewElement.style.width = `${previewRect.width}px`;
    previewElement.style.height = `${previewRect.height}px`;
    previewElement.style.transition = "none";

    const copyProperties = [
      "--tile-size-adjustment",
      "--tile-width",
      "--tile-height",
      "--tile-x",
      "--tile-y",
    ];

    for (const property of copyProperties) {
      // Copy tile styling variables
      previewElement.style.setProperty(
        property,
        computedStyles.getPropertyValue(property),
      );
    }

    // Add preview to DOM
    document.body.append(previewElement);
  }

  function handlePointerMove(event: PointerEvent) {
    if (draggingState === null) return;

    // Update dragging position
    const previewElement = draggingState.previewElement;
    previewElement.style.left = `${event.clientX - draggingState.initialX}px`;
    previewElement.style.top = `${event.clientY - draggingState.initialY}px`;

    // Get elements we are currently hovered over
    const elements = document.elementsFromPoint(event.clientX, event.clientY);
    if (elements.length < 1) {
      dropZoneTarget = null;
      return;
    }

    // Search for a drop zone element
    const dropZoneElement = elements.find(
      (el) => el.hasAttribute("data-drop-zone") && el !== previewElement,
    );
    if (!dropZoneElement) {
      dropZoneTarget = null;
      return;
    }

    // Extract drop zone state
    const dropZone = dropZoneElement.getAttribute("data-drop-zone");
    if (dropZone === "sidebar") {
      dropZoneTarget = {
        type: dropZone,
      };
      return;
    }

    const rowRaw = dropZoneElement.getAttribute("data-row");
    const columnRaw = dropZoneElement.getAttribute("data-column");
    if (dropZone === null || rowRaw === null || columnRaw === null) {
      throw new Error("invalid drop zone, missing required attributes");
    }

    // Set dropzone target
    const row = parseInt(rowRaw);
    const column = parseInt(columnRaw);
    if (dropZone === "filledTile") {
      dropZoneTarget = {
        type: dropZone,
        row,
        column,
      };
    } else if (dropZone === "emptyTile") {
      dropZoneTarget = {
        type: dropZone,
        row,
        column,
      };
    }
  }

  function handlePointerUp() {
    if (draggingState === null) return;

    if (dropZoneTarget !== null) {
      // Dragging in actions and moving existing tiles
      if (dropZoneTarget.type === "emptyTile") {
        const { row, column } = dropZoneTarget;

        if (draggingState.data.type === "tile") {
          const { data } = draggingState;
          onMoveTile(data.tileId, row, column);
        } else if (draggingState.data.type === "action") {
          const { data } = draggingState;
          onPlaceTile(data.pluginId, data.actionId, row, column);
        }
      }
      // Dragged existing tile to sidebar (Removing)
      else if (
        dropZoneTarget.type === "sidebar" &&
        draggingState.data.type === "tile"
      ) {
        const { data } = draggingState;
        onDeleteTile(data.tileId);
      }
    }

    // Reset state
    const previewElement = draggingState.previewElement;
    document.body.removeChild(previewElement);
    draggingState = null;
    dropZoneTarget = null;
  }

  const dragging = $derived(draggingState !== null);

  // Add event listeners to document for better drag handling
  $effect(() => {
    if (dragging) {
      const bodyStyle = getComputedStyle(document.body);
      document.addEventListener("pointermove", handlePointerMove);
      document.addEventListener("pointerup", handlePointerUp);
      document.body.style.overflow = "hidden";

      return () => {
        document.removeEventListener("pointermove", handlePointerMove);
        document.removeEventListener("pointerup", handlePointerUp);
        document.body.style.overflow = bodyStyle.overflow;
      };
    }
  });
</script>

{@render children?.()}

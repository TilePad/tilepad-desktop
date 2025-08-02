<script lang="ts" module>
  export type ResizeSide = "top" | "bottom" | "left" | "right";
</script>

<script lang="ts">
  import {
    resizeHandle,
    ResizeDirection,
    type ResizeEventDetail,
  } from "$lib/utils/resizable";

  type Props = {
    distanceThreshold: number;
    onResize: (detail: ResizeEventDetail, side: ResizeSide) => void;
    onResizeDiagonal: (detail: ResizeEventDetail, sides: ResizeSide[]) => void;
  };

  const { distanceThreshold, onResize, onResizeDiagonal }: Props = $props();

  function handleResizeVerticalTop(detail: ResizeEventDetail) {
    onResize(detail, "top");
  }

  function handleResizeVerticalBottom(detail: ResizeEventDetail) {
    onResize(detail, "bottom");
  }

  function handleResizeHorizontalLeft(detail: ResizeEventDetail) {
    onResize(detail, "left");
  }

  function handleResizeHorizontalRight(detail: ResizeEventDetail) {
    onResize(detail, "right");
  }

  function considerResizeDiagonalLeftTop(detail: ResizeEventDetail) {
    onResizeDiagonal(detail, ["left", "top"]);
  }

  function considerResizeDiagonalLeftBottom(detail: ResizeEventDetail) {
    onResizeDiagonal(detail, ["left", "bottom"]);
  }

  function considerResizeDiagonalRightBottom(detail: ResizeEventDetail) {
    onResizeDiagonal(detail, ["right", "bottom"]);
  }

  function considerResizeDiagonalRightTop(detail: ResizeEventDetail) {
    onResizeDiagonal(detail, ["right", "top"]);
  }
</script>

<span
  class="handle handle--vertical handle--top"
  use:resizeHandle={{
    direction: ResizeDirection.VERTICAL,
    distanceThreshold,
    onResize: handleResizeVerticalTop,
  }}
></span>

<span
  class="handle handle--vertical handle--bottom"
  use:resizeHandle={{
    direction: ResizeDirection.VERTICAL,
    distanceThreshold,
    onResize: handleResizeVerticalBottom,
  }}
></span>

<span
  class="handle handle--horizontal handle--left"
  use:resizeHandle={{
    direction: ResizeDirection.HORIZONTAL,
    distanceThreshold,
    onResize: handleResizeHorizontalLeft,
  }}
></span>

<span
  class="handle handle--corner handle--corner-top-left"
  use:resizeHandle={{
    direction: ResizeDirection.DIAGONAL,
    distanceThreshold,
    onResize: considerResizeDiagonalLeftTop,
  }}
></span>

<span
  class="handle handle--corner handle--corner-bottom-left"
  use:resizeHandle={{
    direction: ResizeDirection.DIAGONAL,
    distanceThreshold,
    onResize: considerResizeDiagonalLeftBottom,
  }}
></span>

<span
  class="handle handle--horizontal handle--right"
  use:resizeHandle={{
    direction: ResizeDirection.HORIZONTAL,
    distanceThreshold,
    onResize: handleResizeHorizontalRight,
  }}
></span>

<span
  class="handle handle--corner handle--corner-top-right"
  use:resizeHandle={{
    direction: ResizeDirection.DIAGONAL,
    distanceThreshold,
    onResize: considerResizeDiagonalRightTop,
  }}
></span>

<span
  class="handle handle--corner handle--corner-bottom-right"
  use:resizeHandle={{
    direction: ResizeDirection.DIAGONAL,
    distanceThreshold,
    onResize: considerResizeDiagonalRightBottom,
  }}
></span>

<style>
  .handle {
    --handle-width: 2px;
    --handle-offset: 1px;
    --handle-corner-size: calc(var(--handle-width) * 2);

    position: absolute;
  }

  .handle:hover {
    background-color: #fff;
  }

  .handle--vertical {
    left: var(--handle-corner-size);
    height: var(--handle-width);
    width: calc(100% - (var(--handle-corner-size) * 2) - var(--handle-offset));
    cursor: row-resize;
  }

  .handle--top {
    top: var(--handle-offset);
  }

  .handle--bottom {
    bottom: var(--handle-offset);
  }

  .handle--horizontal {
    height: calc(100% - (var(--handle-corner-size) * 2) - var(--handle-offset));
    top: var(--handle-corner-size);
    width: var(--handle-width);
    cursor: col-resize;
  }

  .handle--left {
    left: var(--handle-offset);
  }

  .handle--right {
    right: var(--handle-offset);
  }

  .handle--corner {
    height: var(--handle-corner-size);
    width: var(--handle-corner-size);
  }

  .handle--corner-top-left {
    top: var(--handle-offset);
    left: var(--handle-offset);
    cursor: nw-resize;
  }

  .handle--corner-bottom-left {
    bottom: var(--handle-offset);
    left: var(--handle-offset);
    cursor: sw-resize;
  }

  .handle--corner-top-right {
    top: var(--handle-offset);
    right: var(--handle-offset);
    cursor: ne-resize;
  }

  .handle--corner-bottom-right {
    bottom: var(--handle-offset);
    right: var(--handle-offset);
    cursor: se-resize;
  }
</style>

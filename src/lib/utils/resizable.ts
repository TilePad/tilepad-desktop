import type { Action } from "svelte/action";

export type ResizableOptions = {
  direction: ResizeDirection;
  distanceThreshold: number;
};

export type ResizeEventDetail = {
  scaleX: number;
  scaleY: number;
  commit: boolean;
};

export enum ResizeDirection {
  HORIZONTAL,
  VERTICAL,
  DIAGONAL,
}

export const resizeHandle: Action<HTMLElement, ResizableOptions> = (
  node: HTMLElement,
  initialOptions: ResizableOptions,
) => {
  let startPosition = { x: 0, y: 0 };

  let options = initialOptions;

  const getScale = (event: PointerEvent) => {
    const deltaX = event.clientX - startPosition.x;
    const deltaY = event.clientY - startPosition.y;

    let scaleX = 0;
    let scaleY = 0;

    if (options.direction !== ResizeDirection.VERTICAL) {
      scaleX = deltaX / options.distanceThreshold;
    }

    if (options.direction !== ResizeDirection.HORIZONTAL) {
      scaleY = deltaY / options.distanceThreshold;
    }

    return { scaleX: Math.round(scaleX), scaleY: Math.round(scaleY) };
  };

  const handlePointerDown = (event: PointerEvent) => {
    event.preventDefault();
    event.stopPropagation();

    // Capture mouse cursor
    (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);

    // Set initial position
    startPosition = { x: event.clientX, y: event.clientY };

    // Start listening for pointer events
    node.addEventListener("pointermove", handlePointerMove);
    node.addEventListener("pointerup", handlePointerUp);
  };

  const handlePointerMove = (event: PointerEvent) => {
    const scale = getScale(event);
    node.dispatchEvent(
      new CustomEvent("resize", {
        detail: { ...scale, commit: false },
      }),
    );
  };

  const handlePointerUp = (event: PointerEvent) => {
    const scale = getScale(event);
    node.removeEventListener("pointermove", handlePointerMove);
    node.removeEventListener("pointerup", handlePointerUp);
    node.dispatchEvent(
      new CustomEvent("resize", { detail: { ...scale, commit: true } }),
    );
  };

  node.addEventListener("pointerdown", handlePointerDown);
  return {
    update(parameter) {
      options = parameter;
    },
    destroy() {
      node.removeEventListener("pointerdown", handlePointerDown);
      node.removeEventListener("pointermove", handlePointerMove);
      node.removeEventListener("pointerup", handlePointerUp);
    },
  };
};

import { passiveEventArg } from "./browser";

export function createVirtualizer(
  containerState: () => HTMLElement | undefined,
  rowCountState: () => number,
  rowHeightState: () => number,
) {
  const container = $derived.by(containerState);
  const rowHeight = $derived.by(rowHeightState);
  const rowCount = $derived.by(rowCountState);
  const totalHeight = $derived(rowCount * rowHeight);

  // Current scroll offset
  let offset: number = $state(0);
  let containerHeight: number = $state(0);

  function handleScroll(event: Event) {
    const scrollOffset = container?.scrollTop ?? 0;

    if (
      scrollOffset < 0 ||
      offset === scrollOffset ||
      Math.abs(scrollOffset - offset) < rowHeight ||
      event.target !== container
    )
      return;

    requestAnimationFrame(() => {
      offset = scrollOffset;
    });
  }

  $effect(() => {
    if (container === undefined) return;

    // Track container size changes
    const resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        containerHeight = entry.target.clientHeight;
      }
    });

    // Set initial container size
    containerHeight = container.clientHeight;

    resizeObserver.observe(container);

    // Handle container scrolling
    container.addEventListener("scroll", handleScroll, passiveEventArg);

    return () => {
      resizeObserver.disconnect();
      if (container === undefined) return;
      container.removeEventListener("scroll", handleScroll);
    };
  });

  const { startRow, stopRow } = $derived.by(() => {
    if (totalHeight === 0) {
      return {
        startRow: 0,
        stopRow: 0,
      };
    }

    const overscanCount = 2;
    const maxOffset = offset + containerHeight;
    const visibleRow = binarySearch(
      0,
      rowCount,
      Math.max(0, offset),
      rowHeight,
    );

    // Find the first visible row
    const startRow = Math.max(0, visibleRow - overscanCount);

    const startOffset = startRow * rowHeight;
    const visibleHeight = maxOffset - startOffset;
    const visibleRows = Math.ceil(visibleHeight / rowHeight);

    // Determine the last visible row
    const stopRow = Math.min(
      startRow + visibleRows + overscanCount,
      rowCount - 1,
    );

    return {
      startRow,
      stopRow,
    };
  });

  return () => ({ totalHeight, startRow, stopRow });
}

export function binarySearch(
  low: number,
  high: number,
  offset: number,
  itemHeight: number,
) {
  let middle = 0;
  let currentOffset = 0;

  while (low <= high) {
    middle = low + Math.floor((high - low) / 2);
    currentOffset = middle * itemHeight;

    if (currentOffset === offset) {
      return middle;
    } else if (currentOffset < offset) {
      low = middle + 1;
    } else if (currentOffset > offset) {
      high = middle - 1;
    }
  }

  if (low > 0) {
    return low - 1;
  }

  return 0;
}

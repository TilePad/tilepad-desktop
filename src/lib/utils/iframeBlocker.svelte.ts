/**
 * Helper to block clicks to iframes on the page when a certain
 * active condition is met
 *
 * Used by popovers to ensure the interact outside to close behavior
 * works correctly
 *
 * @param active
 */
export function iframeBlocker(active: () => boolean) {
  const activeValue = $derived.by(active);

  $effect(() => {
    const tracked: { iframe: HTMLIFrameElement; originalValue: string }[] = [];

    if (activeValue) {
      const iframes = document.querySelectorAll("iframe");
      for (const frame of iframes) {
        if (frame.style.pointerEvents !== "none") {
          tracked.push({
            iframe: frame,
            originalValue: frame.style.pointerEvents,
          });
          frame.style.pointerEvents = "none";
        }
      }
    }

    return () => {
      tracked.forEach(({ iframe, originalValue }) => {
        iframe.style.pointerEvents = originalValue;
      });
    };
  });
}

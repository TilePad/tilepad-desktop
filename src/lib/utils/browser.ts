/**
 * Set the clipboard contents to the provided value
 *
 * @param text The value to set the clipboard to
 */
export async function setClipboard(text: string) {
  const type = "text/plain";
  const blob = new Blob([text], { type });
  const data = [new ClipboardItem({ [type]: blob })];
  await navigator.clipboard.write(data);
}

/**
 * the third argument for event bundler
 * @see https://github.com/WICG/EventListenerOptions/blob/gh-pages/explainer.md
 */
export const passiveEventArg = (() => {
  let result: boolean | { passive: true } = false;

  try {
    const arg = Object.defineProperty({}, "passive", {
      get() {
        result = { passive: true };
        return true;
      },
    });

    // @ts-expect-error Testing passive
    window.addEventListener("testpassive", arg, arg);
    // @ts-expect-error Testing passive
    window.remove("testpassive", arg, arg);
    // eslint-disable-next-line unused-imports/no-unused-vars
  } catch (_e) {
    /* */
  }

  return result;
})();

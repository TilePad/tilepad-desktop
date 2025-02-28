import { readonly, writable } from "svelte/store";

export function* range(start: number, end: number): Generator<number> {
  for (let i = start; i < end; i++) yield i;
}

/**
 * Translation to convert a callback function that uses
 * runes into a svelte readable store
 *
 * @param cb Callback containing th rune usage
 * @returns Read only access to the store
 */
export function runeStore<T>(cb: () => T) {
  const store = writable<T>();
  $effect.pre(() => {
    store.set(cb());
  });
  return readonly(store);
}

export function persistedState<V>(key: string, initialValue: V) {
  // Load the stored value from localStorage
  let storedValue: V;
  try {
    const item = localStorage.getItem(key);
    storedValue = item ? JSON.parse(item) : initialValue;
  } catch (error) {
    console.error("failed to parse state from local storage", key, error);
    storedValue = initialValue;
  }

  let state = $state<V>(storedValue);

  function onChange(value: V) {
    try {
      localStorage.setItem(key, JSON.stringify(value));
    } catch (error) {
      console.error(
        "failed to persist state to local storage",
        key,
        value,
        error,
      );
    }
  }

  $effect.root(() => {
    $effect(() => {
      onChange(state);
    });

    return () => {};
  });

  return {
    get current() {
      return state;
    },
    set current(value: V) {
      state = value;
    },
  };
}

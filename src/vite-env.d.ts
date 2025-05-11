/// <reference types="svelte" />
/// <reference types="vite/client" />
/// <reference types="unplugin-icons/types/svelte" />

import type { ResizeEventDetail } from "$lib/utils/resizable";

// Add custom event handlers
declare global {
  declare namespace svelteHTML {
    interface HTMLAttributes {
      onresize?: (e: CustomEvent<ResizeEventDetail>) => void;
    }
  }
}

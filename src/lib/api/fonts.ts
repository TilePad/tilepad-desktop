import { invoke } from "@tauri-apps/api/core";
import { createQuery } from "@tanstack/svelte-query";

const fontsKeys = {
  root: ["fonts"],
  list: ["fonts", "list"],
};

// [REQUESTS] ------------------------------------------------------

function getFonts() {
  return invoke<string[]>("fonts_fonts");
}

// [QUERIES] ------------------------------------------------------

export function createFontsQuery() {
  return createQuery({
    queryKey: fontsKeys.list,
    queryFn: () => getFonts(),
  });
}

// [MUTATIONS] ------------------------------------------------------

// [MUTATORS] ------------------------------------------------------

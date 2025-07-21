import { createQuery } from "@tanstack/svelte-query";

import { fontsKeys } from "./fonts.keys";
import { getFonts } from "./fonts.requests";

export function createFontsQuery() {
  return createQuery({
    queryKey: fontsKeys.list,
    queryFn: getFonts,
  });
}

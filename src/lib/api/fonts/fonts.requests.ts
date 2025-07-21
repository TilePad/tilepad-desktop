import { invoke } from "@tauri-apps/api/core";

export function getFonts() {
  return invoke<string[]>("fonts_fonts");
}

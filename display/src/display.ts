import "./styles.css";
import tile from "./tile";
import plugin from "./plugin";

const tilepad = { tile, plugin };

type Tilepad = typeof tilepad;

declare global {
  var tilepad: Tilepad;
}

globalThis.tilepad = tilepad;

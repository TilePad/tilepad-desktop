import { inspector } from "./events";
import { asyncEventCallback } from "./utils";
import { DebouncedPropertyUpdater } from "./debouncePropertyUpdater";

function setProperties(properties: unknown, partial: boolean = true) {
  inspector.send({
    type: "SET_PROPERTIES",
    properties,
    partial,
  });
}

const debouncedUpdater = new DebouncedPropertyUpdater(setProperties, 100, 1500);
const setProperty = debouncedUpdater.setProperty.bind(debouncedUpdater);

const tile: TileApi = {
  requestTile(): void {
    inspector.send({
      type: "GET_TILE",
    });
  },

  getTile(): Promise<Tile> {
    return asyncEventCallback(tile.onTile, tile.requestTile);
  },

  onTile: (callback: (tile: Tile) => void): DisposeFunction => {
    return inspector.subscribe("tile", callback);
  },

  requestProperties() {
    inspector.send({
      type: "GET_PROPERTIES",
    });
  },

  onProperties(callback: (properties: unknown) => void): DisposeFunction {
    return inspector.subscribe("properties", callback);
  },

  getProperties(): Promise<unknown> {
    return asyncEventCallback(tile.onProperties, tile.requestProperties);
  },

  setProperty,
  setProperties,

  setLabel(label: TilepadLabel): void {
    inspector.send({
      type: "SET_LABEL",
      label,
    });
  },

  setIcon(icon: TilepadIcon): void {
    inspector.send({
      type: "SET_ICON",
      icon,
    });
  },
};

export default tile;

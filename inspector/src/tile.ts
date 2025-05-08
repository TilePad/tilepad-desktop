import { debounce } from "./utils";
import { inspector } from "./events";

interface Tile {
  profileId: string;
  folderId: string;
  pluginId: string;
  tileId: string;
  actionId: string;
  properties: unknown;
}

export type TilepadLabel = Partial<{
  enabled: boolean;
  label: string;
  align: "Bottom" | "Middle" | "Top";
  font_size: number;
  bold: boolean;
  italic: boolean;
  underline: boolean;
  outline: boolean;
  color: string;
  outline_color: string;
}>;

export type TilepadIcon =
  | {
      type: "None";
    }
  | { type: "PluginIcon"; plugin_id: string; icon: string }
  | { type: "IconPack"; pack_id: string; path: string }
  | { type: "Url"; src: string };

const tile = {
  /**
   * Request the current tile details
   */
  requestTile() {
    inspector.send({
      type: "GET_TILE",
    });
  },

  /**
   * Get the current tile details
   */
  getTile(): Promise<Tile> {
    return new Promise((resolve) => {
      const dispose = tile.onTile((tile) => {
        resolve(tile);
        dispose();
      });
      tile.requestTile();
    });
  },

  /**
   * Subscribes to tile, will receive the outcome
   * of {@link Tilepad.requestTile}
   *
   * The returned function can be used to remove the subscription
   *
   * @param callback The callback to invoke when a message is received
   * @returns Function that will remove the listener when called
   */
  onTile: (callback: (tile: Tile) => void) => {
    inspector.on("tile", callback);
    return () => {
      inspector.off("tile", callback);
    };
  },

  /**
   * Requests the current properties for the tile.
   * When the properties are received {@link Tilepad.onProperties}
   * will be run
   */
  requestProperties() {
    inspector.send({
      type: "GET_PROPERTIES",
    });
  },

  /**
   * Subscribes to properties for the tile, will receive the outcome
   * of {@link Tilepad.requestProperties}
   *
   * The returned function can be used to remove the subscription
   *
   * @param callback The callback to invoke when a message is received
   * @returns Function that will remove the listener when called
   */
  onProperties(callback: (properties: unknown) => void) {
    inspector.on("properties", callback);
    return () => {
      inspector.off("properties", callback);
    };
  },

  /**
   * Requests the current properties waiting until they're
   * obtained returning a promise that resolves with the
   * properties
   */
  getProperties(): Promise<unknown> {
    return new Promise((resolve) => {
      const dispose = tile.onProperties((properties) => {
        resolve(properties);
        dispose();
      });
      tile.requestProperties();
    });
  },

  /**
   * Set a property within the tile properties
   *
   * @param name The name of the property to set
   * @param value The value of the property
   */
  setProperty: debounce((name: string, value: unknown) => {
    tile.setProperties({ [name]: value });
  }, 100),

  /**
   * Sets the properties of the tile.
   *
   * This is a partial update, only the provided parts
   * of the object will be updated, anything not specified
   * already existing in the tile properties will continue
   * to exist
   *
   * @param properties The partial tile properties data
   */
  setProperties(properties: unknown) {
    inspector.send({
      type: "SET_PROPERTIES",
      properties,
    });
  },

  /**
   * Set the current label of the tile. Will not
   * work if the user has already specified a label
   * user must make their label blank for this to apply
   *
   * @param label
   */
  setLabel(label: TilepadLabel) {
    inspector.send({
      type: "SET_LABEL",
      label,
    });
  },

  /**
   * Set the current icon of the tile. Will not
   * work if the user has already specified a icon
   * user must set their icon to None for this to apply
   *
   * @param icon
   */
  setIcon(icon: TilepadIcon) {
    inspector.send({
      type: "SET_ICON",
      icon,
    });
  },
};

export default tile;

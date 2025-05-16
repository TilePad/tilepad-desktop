import { display } from "./events";

interface Tile {
  pluginId: string;
  tileId: string;
  actionId: string;
}

const tile = {
  /**
   * Request the current tile details
   */
  requestTile() {
    display.send({
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
    display.on("tile", callback);
    return () => {
      display.off("tile", callback);
    };
  },
};

export default tile;

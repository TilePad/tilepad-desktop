import { display } from "./events";
import { asyncEventCallback } from "./utils";

const tile: TileApi = {
  requestTile(): void {
    display.send({
      type: "GET_TILE",
    });
  },

  getTile(): Promise<Tile> {
    return asyncEventCallback(this.onTile, this.requestTile);
  },

  onTile: (callback: (tile: Tile) => void): DisposeFunction => {
    return display.subscribe("tile", callback);
  },
};

export default tile;

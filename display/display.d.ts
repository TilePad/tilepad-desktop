export interface PluginApi {
  /**
   * Send a message to the plugin
   *
   * @param message The message to send
   */
  send(message: unknown): void;
  /**
   * Subscribes to messages sent to the inspector via the
   * associated plugin for the action
   *
   * The returned function can be used to remove the subscription
   *
   * @param callback The callback to invoke when a message is received
   * @returns Function that will remove the listener when called
   */
  onMessage(callback: (message: unknown) => void): VoidFunction;
}

interface Tile {
  pluginId: string;
  tileId: string;
  actionId: string;
}

export interface TileApi {
  /**
   * Request the current tile details
   */
  requestTile(): void;
  /**
   * Get the current tile details
   */
  getTile(): Promise<Tile>;
  /**
   * Subscribes to tile, will receive the outcome
   * of {@link Tilepad.requestTile}
   *
   * The returned function can be used to remove the subscription
   *
   * @param callback The callback to invoke when a message is received
   * @returns Function that will remove the listener when called
   */
  onTile(callback: (tile: Tile) => void): VoidFunction;
}

export interface Tilepad {
  tile: TileApi;
  plugin: PluginApi;
}

declare global {
  var tilepad: Tilepad;
}

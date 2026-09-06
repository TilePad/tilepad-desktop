declare global {
  type DisposeFunction = VoidFunction;

  interface PluginApi {
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
    onMessage(callback: (message: unknown) => void): DisposeFunction;
  }

  interface Tile {
    pluginId: string;
    tileId: string;
    actionId: string;
  }

  interface TileApi {
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
     * of {@link TileApi.requestTile}
     *
     * The returned function can be used to remove the subscription
     *
     * @param callback The callback to invoke when a message is received
     * @returns Function that will remove the listener when called
     */
    onTile(callback: (tile: Tile) => void): DisposeFunction;
  }

  interface TilepadApi {
    tile: TileApi;
    plugin: PluginApi;
  }

  var tilepad: TilepadApi;
}
export {};

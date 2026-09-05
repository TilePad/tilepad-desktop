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

    /**
     * Requests the current properties for the plugin.
     * When the properties are received {@link PluginApi.onProperties}
     * will be run
     */
    requestProperties(): void;

    /**
     * Subscribes to properties for the plugin, will receive the outcome
     * of {@link PluginApi.requestProperties}
     *
     * @param callback The callback to invoke when a message is received
     * @returns Disposal function to remove the properties subscription
     */
    onProperties(callback: (properties: unknown) => void): DisposeFunction;

    /**
     * Requests the current plugin properties waiting until they're
     * obtained returning a promise that resolves with the properties
     *
     * Helper for using {@link PluginApi.requestProperties} and {@link PluginApi.onProperties}
     * to asynchronously fetch the current properties
     */
    getProperties(): Promise<unknown>;

    /**
     * Sets the properties of the plugin.
     *
     * By default this is a partial update, only the provided parts
     * of the object will be updated, anything not specified already
     * existing in the tile properties will continue to exist
     *
     * @param properties The partial tile properties data
     * @param partial Whether to perform a partial update or to replace the entire properties set
     */
    setProperties(properties: unknown, partial: boolean = true): void;

    /**
     * Set a single property on the plugin properties object
     *
     * Calls to this function are debounced by 100ms, with a maximum
     * delay of 1500ms which will cause the update to be applied regardless
     * of the debounce
     *
     * @param name The name of the property key to set
     * @param value The new property value
     */
    setProperty(name: string, value: unknown): void;
  }

  interface Tile {
    profileId: string;
    folderId: string;
    pluginId: string;
    tileId: string;
    actionId: string;
    properties: unknown;
  }

  type TilepadLabelAlign = "Bottom" | "Middle" | "Top" | string;

  type TilepadLabel = Partial<{
    enabled: boolean;
    label: string;
    align: TilepadLabelAlign;
    font_size: number;
    bold: boolean;
    italic: boolean;
    underline: boolean;
    outline: boolean;
    color: string;
    outline_color: string;
  }>;

  export type TilepadIcon =
    | { type: "None" }
    | { type: "PluginIcon"; plugin_id: string; icon: string }
    | { type: "IconPack"; pack_id: string; path: string }
    | { type: "Url"; src: string };

  interface TileApi {
    /**
     * Request the current tile details
     */
    requestTile(): void;
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
    /**
     * Get the current tile details of the tile associated to
     * this inspector window
     *
     * Helper for using {@link TileApi.requestTile} and {@link TileApi.onTile}
     * to asynchronously fetch the current tile
     */
    getTile(): Promise<Tile>;

    /**
     * Requests the current properties for the tile.
     * When the properties are received {@link TileApi.onProperties}
     * will be run
     */
    requestProperties(): void;

    /**
     * Subscribes to properties for the tile, will receive the outcome
     * of {@link TileApi.requestProperties}
     *
     * @param callback The callback to invoke when a message is received
     * @returns Disposal function to remove the properties subscription
     */
    onProperties(callback: (properties: unknown) => void): DisposeFunction;

    /**
     * Requests the current tile properties waiting until they're
     * obtained returning a promise that resolves with the properties
     *
     * Helper for using {@link TileApi.requestProperties} and {@link TileApi.onProperties}
     * to asynchronously fetch the current properties
     */
    getProperties(): Promise<unknown>;

    /**
     * Sets the properties of the tile.
     *
     * This is a partial update, only the provided parts
     * of the object will be updated, anything not specified
     * already existing in the tile properties will continue
     * to exist
     *
     * @param properties The partial tile properties data
     * @param partial Whether to perform a partial update or to replace the entire properties set
     */
    setProperties(properties: unknown, partial: boolean = true): void;

    /**
     * Set a property within the tile properties
     *
     * Calls to this function are debounced by 100ms, with a maximum
     * delay of 1500ms which will cause the update to be applied regardless
     * of the debounce
     *
     * @param name The name of the property to set
     * @param value The value of the property
     */
    setProperty(name: string, value: unknown): void;

    /**
     * Set the current label of the tile.
     *
     * This will only update the label if the user has not already
     * specified a custom label for the tile. If the user has
     * a custom label they will need to make the label blank for
     * this function to work.
     *
     * @param label The new label data
     */
    setLabel(label: TilepadLabel): void;

    /**
     * Set the current icon of the tile.
     *
     * This will only update the label if the user has not already
     * specified a custom icon for the tile. If the user has
     * a custom icon they will need to remove the icon for
     * this function to work.
     *
     * @param icon The new icon data
     */
    setIcon(icon: TilepadIcon): void;
  }

  interface TilepadApi {
    tile: TileApi;
    plugin: PluginApi;
  }

  var tilepad: TilepadApi;
}

// Required for typescript to treat this file as a real module
export {};

export interface TilepadPlugin {
  /**
   * Send a message to the plugin
   *
   * @param message The message to send
   */
  send(message: unknown);

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

export interface TilepadTile {
  /**
   * Requests the current properties for the tile.
   * When the properties are received {@link Tilepad.onProperties}
   * will be run
   */
  requestProperties();

  /**
   * Requests the current properties waiting until they're
   * obtained returning a promise that resolves with the
   * properties
   */
  getProperties(): Promise<unknown>;

  /**
   * Set a property within the tile properties
   *
   * @param name The name of the property to set
   * @param value The value of the property
   */
  setProperty(name: string, value: unknown): VoidFunction;

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
  setProperties(properties: unknown): VoidFunction;

  /**
   * Subscribes to messages sent to the inspector via the
   * associated plugin for the action
   *
   * The returned function can be used to remove the subscription
   *
   * @param callback The callback to invoke when a message is received
   * @returns Function that will remove the listener when called
   */
  onProperties(callback: (properties: unknown) => void): VoidFunction;

  /**
   * Set the current label of the tile. Will not
   * work if the user has already specified a label
   * user must make their label blank for this to apply
   *
   * @todo Not implemented yet
   *
   * @param label
   */
  setLabel(label: TilepadLabel);

  /**
   * Set the current icon of the tile. Will not
   * work if the user has already specified a icon
   * user must set their icon to None for this to apply
   *
   * @todo Not implemented yet
   *
   * @param icon
   */
  setIcon(icon: TilepadIcon);
}

export type TilepadLabel = Partial<{
  enabled: boolean;
  label: string;
  align: "Bottom" | "Middle" | "Top";
  font_size: number;
  bold: boolean;
  italic: boolean;
  underline: boolean;
  color: string;
}>;

export type TilepadIcon =
  | {
      type: "None";
    }
  | { type: "PluginIcon"; plugin_id: string; icon: string }
  | { type: "IconPack"; pack_id: string; path: string }
  | { type: "Url"; src: string };

export interface Tilepad {
  /**
   * Access to the plugin for the tile action
   */
  plugin: TilepadPlugin;

  /**
   * Access to the tile itself
   */
  tile: TilepadTile;
}

declare global {
  interface Window {
    tilepad: Tilepad;
  }
}

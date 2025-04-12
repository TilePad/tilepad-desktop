import { debounce } from "./utils";
import { inspector } from "./events";

const plugin = {
  /**
   * Send a message to the plugin
   *
   * @param message The message to send
   */
  send(message: unknown) {
    inspector.send({
      type: "SEND_TO_PLUGIN",
      message,
    });
  },

  /**
   * Subscribes to messages sent to the inspector via the
   * associated plugin for the action
   *
   * The returned function can be used to remove the subscription
   *
   * @param callback The callback to invoke when a message is received
   * @returns Function that will remove the listener when called
   */
  onMessage(callback: (message: unknown) => void) {
    inspector.on("plugin_message", callback);
    return () => {
      inspector.off("plugin_message", callback);
    };
  },

  requestProperties() {
    inspector.send({
      type: "GET_PLUGIN_PROPERTIES",
    });
  },

  onProperties(callback: (properties: unknown) => void) {
    inspector.on("plugin_properties", callback);
    return () => {
      inspector.off("plugin_properties", callback);
    };
  },

  getProperties(): Promise<unknown> {
    return new Promise((resolve) => {
      const dispose = plugin.onProperties((properties) => {
        resolve(properties);
        dispose();
      });
      plugin.requestProperties();
    });
  },

  setProperty: debounce((name: string, value: unknown) => {
    plugin.setProperties({ [name]: value });
  }, 100),

  setProperties(properties: unknown) {
    inspector.send({
      type: "SET_PLUGIN_PROPERTIES",
      properties,
    });
  },
};

export default plugin;

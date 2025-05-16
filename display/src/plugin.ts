import { display } from "./events";

const plugin = {
  /**
   * Send a message to the plugin
   *
   * @param message The message to send
   */
  send(message: unknown) {
    display.send({
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
    display.on("plugin_message", callback);
    return () => {
      display.off("plugin_message", callback);
    };
  },
};

export default plugin;

import { display } from "./events";

const plugin: PluginApi = {
  send(message: unknown): void {
    display.send({
      type: "SEND_TO_PLUGIN",
      message,
    });
  },

  onMessage(callback: (message: unknown) => void): DisposeFunction {
    return display.subscribe("plugin_message", callback);
  },
};

export default plugin;

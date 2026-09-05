import { inspector } from "./events";
import { asyncEventCallback } from "./utils";
import { DebouncedPropertyUpdater } from "./debouncePropertyUpdater";

function setProperties(properties: unknown, partial: boolean = true) {
  inspector.send({
    type: "SET_PLUGIN_PROPERTIES",
    properties,
    partial,
  });
}

const debouncedUpdater = new DebouncedPropertyUpdater(setProperties, 100, 1500);
const setProperty = debouncedUpdater.setProperty.bind(debouncedUpdater);

const plugin: PluginApi = {
  send(message: unknown) {
    inspector.send({
      type: "SEND_TO_PLUGIN",
      message,
    });
  },

  onMessage(callback: (message: unknown) => void) {
    return inspector.subscribe("plugin_message", callback);
  },

  requestProperties() {
    inspector.send({ type: "GET_PLUGIN_PROPERTIES" });
  },

  onProperties(callback: (properties: unknown) => void): DisposeFunction {
    return inspector.subscribe("plugin_properties", callback);
  },

  getProperties(): Promise<unknown> {
    return asyncEventCallback(plugin.onProperties, plugin.requestProperties);
  },

  setProperty,
  setProperties,
};

export default plugin;

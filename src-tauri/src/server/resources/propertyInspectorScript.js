/* eslint-disable */
// @ts-nocheck

class EventEmitter {
  constructor() {
    this.events = {};
  }

  // Subscribe to an event
  on(event, callback) {
    if (!this.events[event]) {
      this.events[event] = [];
    }
    this.events[event].push(callback);
  }

  // Unsubscribe from an event
  off(event, callback) {
    if (!this.events[event]) return;
    this.events[event] = this.events[event].filter((cb) => cb !== callback);
  }

  // Emit an event
  emit(event, ...args) {
    if (!this.events[event]) return;
    this.events[event].forEach((callback) => callback(...args));
  }
}

const events = new EventEmitter();

let propertiesListeners = [];
let pluginMessageListeners;

window.addEventListener("message", (event) => {
  const data = event.data;
  const type = data.type;

  if (type === "PROPERTIES") {
    events.emit("properties", data.properties, {
      tileId: data.tileId,
      actionId: data.actionId,
    });
  } else if (type === "PLUGIN_MESSAGE") {
    events.emit("plugin_message", data.message);
  } else if (type === "REFRESH") {
    window.location.reload();
  }
});

function debounce(fn, delay) {
  let timeoutId;

  return function (...args) {
    // Clear the previous timeout
    if (timeoutId) {
      clearTimeout(timeoutId);
    }

    // Set a new timeout with the specified delay
    timeoutId = setTimeout(() => {
      fn.apply(this, args);
    }, delay);
  };
}

function postInspectorMessage(msg) {
  window.parent.postMessage(msg, "*");
}

function requestProperties() {
  postInspectorMessage({
    type: "GET_PROPERTIES",
  });
}

function setProperty(name, value) {
  setProperties({ [name]: value });
}

function setProperties(properties) {
  postInspectorMessage({
    type: "SET_PROPERTIES",
    properties,
  });
}

function sendPluginMessage(message) {
  postInspectorMessage({
    type: "SEND_TO_PLUGIN",
    message,
  });
}

function onPluginMessage(callback) {
  events.on("plugin_message", callback);
  return () => {
    events.off("plugin_message", callback);
  };
}

function onProperties(callback) {
  events.on("properties", callback);
  return () => {
    events.off("properties", callback);
  };
}

function setLabel() {

}

function setIcon() {

}

window.tilepad = {
  onPluginMessage,
  onProperties,

  requestProperties,
  sendPluginMessage,
  setProperty: debounce(setProperty, 100),
  setProperties,
  setLabel,
  setIcon
};

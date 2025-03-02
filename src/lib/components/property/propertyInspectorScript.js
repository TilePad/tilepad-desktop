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
    events.emit("properties", data.properties);
  } else if (type === "PLUGIN_MESSAGE") {
    events.emit("plugin_message", data.message);
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

function requestProperties() {
  window.parent.postMessage({
    type: "GET_PROPERTIES",
  });
}

function setProperty(name, value) {
  window.parent.postMessage({
    type: "SET_PROPERTY",
    name,
    value,
  });
}

function sendPluginMessage(message) {
  window.parent.postMessage({
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

window.tilepad = {
  onPluginMessage,
  onProperties,

  requestProperties,
  sendPluginMessage,
  setProperty: debounce(setProperty, 100),
};

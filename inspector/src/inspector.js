/* eslint-disable */
// @ts-nocheck

import "./styles.css"
import { EventEmitter } from "./eventEmitter";
import { debounce } from "./utils";

const events = new EventEmitter();

window.addEventListener("message", (event) => {
    const data = event.data;
    const type = data.type;

    // Handled when properties are received
    if (type === "PROPERTIES") {
        events.emit("properties", data.properties, {
            tileId: data.tileId,
            actionId: data.actionId,
        });
    }
    // Handled when a message comes in from the plugin
    else if (type === "PLUGIN_MESSAGE") {
        events.emit("plugin_message", data.message);
    }
    // Used by the inspector to force a refresh for new state
    else if (type === "REFRESH") {
        window.location.reload();
    }
});

function postInspectorMessage(msg) {
    window.parent.postMessage(msg, "*");
}

/**
 * @type {TilepadTile}
 */
const tile = {
    requestProperties: () => {
        postInspectorMessage({
            type: "GET_PROPERTIES",
        });
    },
    getProperties: () => {
        return new Promise((resolve) => {
            let dispose;
            dispose = tile.onProperties((properties) => {
                resolve(properties);
                dispose();
            })
            tile.requestProperties();
        })
    },
    setProperty: debounce((name, value) => {
        tile.setProperties({ [name]: value });
    }, 100),
    setProperties: (properties) => {
        postInspectorMessage({
            type: "SET_PROPERTIES",
            properties,
        });
    },
    onProperties: (callback) => {
        events.on("properties", callback);
        return () => {
            events.off("properties", callback);
        };
    },
    setLabel: (label) => { },
    setIcon: (icon) => { }
}

/**
 * @type {TilepadPlugin}
 */
const plugin = {
    send: (message) => {
        postInspectorMessage({
            type: "SEND_TO_PLUGIN",
            message,
        });
    },
    onMessage: (callback) => {
        events.on("plugin_message", callback);
        return () => {
            events.off("plugin_message", callback);
        };
    }
}

/**
 * @type {Tilepad}
 */
const tilepad = { tile, plugin }

window.tilepad = tilepad;

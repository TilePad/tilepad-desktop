import { EventEmitter } from "./eventEmitter";

class Display extends EventEmitter {
  constructor() {
    super();
    window.addEventListener("message", this.onWindowMessage.bind(this));
  }

  /**
   * Handle messages from the window
   *
   * @param event
   */
  private onWindowMessage(event: MessageEvent): void {
    const data = event.data;
    const type = data.type;

    // Handled when the tile is received
    if (type === "TILE") {
      this.emit("tile", data.tile);
    }
    // Handled when a message comes in from the plugin
    else if (type === "PLUGIN_MESSAGE") {
      this.emit("plugin_message", data.message);
    }
    // Used by the inspector to force a refresh for new state
    else if (type === "REFRESH") {
      window.location.reload();
    }
  }

  /**
   * Sends a message to the plugin
   *
   * @param msg The message to send
   */
  send(msg: unknown): void {
    window.parent.postMessage(msg, "*");
  }
}

export const display = new Display();

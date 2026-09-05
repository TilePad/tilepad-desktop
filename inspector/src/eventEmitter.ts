/* eslint-disable @typescript-eslint/no-explicit-any */

type EventCallback = (...args: any[]) => void;

/**
 * Event emitting and subscribing
 */
export class EventEmitter {
  private events: Record<string, EventCallback[]> = {};

  // Subscribe to an event
  on<T extends EventCallback>(event: string, callback: T) {
    if (!this.events[event]) {
      this.events[event] = [];
    }
    this.events[event].push(callback);
  }

  // Unsubscribe from an event
  off<T extends EventCallback>(event: string, callback: T) {
    if (!this.events[event]) return;
    this.events[event] = this.events[event].filter((cb) => cb !== callback);
  }

  // Subscribe to an event returning a dispose function to remove the subscription
  subscribe<T extends EventCallback>(event: string, callback: T) {
    this.on(event, callback);

    return () => {
      this.off(event, callback);
    };
  }

  // Emit an event
  emit(event: string, ...args: any[]) {
    if (!this.events[event]) return;
    this.events[event].forEach((callback) => callback(...args));
  }
}

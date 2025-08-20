/* eslint-disable @typescript-eslint/no-explicit-any */

type EventCallback = (...args: any[]) => void;

/**
 * Event emitting and subscribing
 */
export class EventEmitter {
  private events: Record<string, EventCallback[]>;

  constructor() {
    this.events = {};
  }

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

  // Emit an event
  emit(event: string, ...args: any[]) {
    if (!this.events[event]) return;
    this.events[event].forEach((callback) => callback(...args));
  }
}

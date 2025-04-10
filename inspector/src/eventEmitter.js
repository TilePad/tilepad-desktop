/**
 * Event emitting and subscribing 
 */
export class EventEmitter {
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

type TimeoutId = ReturnType<typeof setTimeout>;

/**
 * Helper to wrap a setProperties function to produce a setProperty function that can
 * be called many times while having a debounce to prevent spamming and a max wait to
 * ensure messages will still be sent after a reasonable time even if a debounce has
 * not been reached
 *
 */
export class DebouncedPropertyUpdater {
  private setProperties: (properties: unknown) => void;
  private debounce: number;
  private maxWait: number;

  private pendingUpdates: Record<string, unknown> = {};

  private debounceTimeoutId: TimeoutId | undefined;
  private maxWaitTimeoutId: TimeoutId | undefined;

  /**
   *
   * @param setProperties Function to set the actual properties
   * @param debounce Debounce for calls to setProperties
   * @param maxWait Max time to debounce for before calling setProperties anyway
   */
  constructor(
    setProperties: (properties: unknown) => void,
    debounce: number,
    maxWait: number,
  ) {
    this.setProperties = setProperties;
    this.debounce = debounce;
    this.maxWait = maxWait;

    this.flushInternal = this.flushInternal.bind(this);
  }

  setProperty(name: string, value: unknown) {
    this.pendingUpdates[name] = value;
    this.flush();
  }

  private flushInternal() {
    this.clearTimers(); // Reset all timers on flush

    this.setProperties(this.pendingUpdates);
    this.pendingUpdates = {};
  }

  private setDebounceTimeoutId(timeoutId: TimeoutId | undefined) {
    if (this.debounceTimeoutId) clearTimeout(this.debounceTimeoutId);
    this.debounceTimeoutId = timeoutId;
  }

  private setMaxWaitTimeoutId(timeoutId: TimeoutId | undefined) {
    if (this.maxWaitTimeoutId) clearTimeout(this.maxWaitTimeoutId);
    this.maxWaitTimeoutId = timeoutId;
  }

  private clearTimers() {
    this.setDebounceTimeoutId(undefined);
    this.setMaxWaitTimeoutId(undefined);
  }

  private flush() {
    // Replace the debounce timer with a new one for our current call
    const flushTimeout = setTimeout(this.flushInternal, this.debounce);
    this.setDebounceTimeoutId(flushTimeout);

    // Eager flush if maximum timeout is exceed
    if (!this.maxWaitTimeoutId) {
      this.maxWaitTimeoutId = setTimeout(this.flushInternal, this.maxWait);
    }
  }
}

/**
 * Subscribes to an event handler, triggers the event, waits for the result, then disposes
 * the event handler and resolves the value
 *
 * @param onEvent Event handler to subscribe to with our async callback
 * @param trigger The trigger to initiate the event (i.e fetch data that will run the handler)
 * @returns The promise for the result value
 */
export function asyncEventCallback<T>(
  onEvent: (callback: (value: T) => void) => DisposeFunction,
  trigger: () => void,
): Promise<T> {
  return new Promise((resolve) => {
    const dispose = onEvent((value) => {
      resolve(value);
      dispose();
    });

    trigger();
  });
}

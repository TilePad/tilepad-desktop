
/**
 * Helper to debounce calls to a function to ensure that
 * a delay has elapsed between calls
 * 
 * @param {*} fn The function to call
 * @param {*} delay The delay to wait before calling (Reset if called before the delay has elapsed)
 * @returns The debounced function
 */
export function debounce(fn, delay) {
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

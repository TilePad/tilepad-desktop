/* eslint-disable */
// @ts-nocheck

window.addEventListener("message", (event) => {
  document.body.innerHTML += "<p>Received from parent: " + event.data + "</p>";
});

window.parent.postMessage("Hello from Iframe");

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

function setSetting(name, value) {}

window.tilepad = {
  setSetting: debounce(setSetting, 100),
};

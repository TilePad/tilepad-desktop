/* eslint-disable */
// @ts-nocheck

const channel = new BroadcastChannel("TILEPAD_INSPECTOR");

channel.onmessage = (event) => {
  document.body.innerHTML += "<p>Received from parent: " + event.data + "</p>";
};

channel.postMessage("Hello from Iframe");

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

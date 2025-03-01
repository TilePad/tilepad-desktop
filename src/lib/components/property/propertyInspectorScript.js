/* eslint-disable */
// @ts-nocheck

const channel = new BroadcastChannel("TILEPAD_INSPECTOR");

channel.onmessage = (event) => {
  document.body.innerHTML += "<p>Received from parent: " + event.data + "</p>";
};

channel.postMessage("Hello from Iframe");

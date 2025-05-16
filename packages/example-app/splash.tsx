
import { invoke } from '@tauri-apps/api/core';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { prepare, start } from '@inkibra/tauri-plugin-ota';



function render() {
  const outlet = document.getElementById('inkibra-tauri-plugin-testbed-app-outlet');
  if (!outlet) {
    throw new Error('No outlet found');
  }
  ReactDOM.createRoot(outlet).render(
    <React.StrictMode>
      <h1>Splash Screen</h1>

      <button onClick={async () => {
        const prepareResponse = await prepare('https://github.com/inKibra/tauri-plugins/releases/download/ota-update/manifest-example.json');
        console.log('prepareResponse', prepareResponse);
      }}>Prepare (Valid Update)</button>
      <button onClick={async () => {
        const prepareResponse = await prepare('https://github.com/inKibra/tauri-plugins/releases/download/ota-update/manifest-example-invalid.json');
        console.log('prepareResponse', prepareResponse);
      }}>Prepare (Invalid Update)</button>
      <button onClick={async () => {
        const startResponse = await start();
        console.log('startResponse', startResponse);
      }}>Start</button>
    </React.StrictMode>,
  );
}

window.addEventListener('unhandledrejection', (event) => {
  console.error(
    'Unhandled rejection (promise: ',
    event.promise,
    ', reason: ',
    event.reason,
    ').',
  );
});

window.onload = async () => {
  console.log('Rust console attached.');
  console.log(await invoke('greet', { name: 'inKibra' }));
  render();
};


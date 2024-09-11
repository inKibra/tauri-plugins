
import { invoke } from '@tauri-apps/api/core';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { attachConsole } from '@tauri-apps/plugin-log';
// import { ping, share } from '@inkibra/tauri-plugin-sharing';
import { showContextMenu } from '@inkibra/tauri-plugin-context-menu';
import { showMap } from '@inkibra/tauri-plugin-map-display';



function render() {
  const outlet = document.getElementById('inkibra-tauri-plugin-testbed-app-outlet');
  if (!outlet) {
    throw new Error('No outlet found');
  }
  ReactDOM.createRoot(outlet).render(
    <React.StrictMode>
      <h1>Hello World</h1>
      <button onClick={async () => {
        const selectedId = await showContextMenu([{
          title: 'Hello',
          id: 'hello',
        }], 0, 0);
        console.log('selectedId', selectedId);
      }}>Show Context Menu</button>
      <button onClick={async () => {
        const showMapResponse = await showMap({
          region: {
            latitude: 37.7749295,
            longitude: -122.419416,
            latitudeDelta: 0.01,
            longitudeDelta: 0.01,
          },
          style: 'dark',
          mapType: 'mutedStandard',
          poiFilter: ['landmark'],
          showBuildings: true,
          showTraffic: false,
          cameraPitch: 45,
        });
        console.log('showMapResponse', showMapResponse);
      }}>Show Map</button>
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
  await attachConsole();
  console.log('Rust console attached.');
  console.log(await invoke('greet', { name: 'inKibra' }));
  render();
  console.log('render() finished.');
  // console.log(await ping('Pong!'));
  // console.log(await share('Check out ToneTempo on the App Store!', 'https://apps.apple.com/us/app/tonetempo/id6471622223'));
};

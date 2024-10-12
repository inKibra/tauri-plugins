
import { invoke } from '@tauri-apps/api/core';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { attachConsole } from '@tauri-apps/plugin-log';
import { showContextMenu } from '@inkibra/tauri-plugin-context-menu';
import { showMap } from '@inkibra/tauri-plugin-map-display';
import { impactFeedback } from '@inkibra/tauri-plugin-haptic-feedback';
import { requestPermissions, checkPermissions, watchPosition, getCurrentPosition } from '@inkibra/tauri-plugin-geolocation';
import { purchaseProduct, restorePurchases, fetchProducts } from '@inkibra/tauri-plugin-iap';
import { authenticate } from '@inkibra/tauri-plugin-auth';

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
        }]);
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
      <button onClick={async () => {
        const impactFeedbackResponse = await impactFeedback('heavy');
        console.log('impactFeedbackResponse', impactFeedbackResponse);
      }}>Impact Feedback</button>
      <button onClick={async () => {
        let permissionsStatus = await checkPermissions();
        if (permissionsStatus.location !== 'granted') {
          permissionsStatus = await requestPermissions(['location'], true);
        }
        if (permissionsStatus.location === 'denied') {
          console.log('Location permission denied');
          return;
        }
        const currentPosition = await getCurrentPosition();
        console.log('currentPosition', currentPosition);
      }}>Get Current Position</button>
      <button onClick={async () => {
        const watchId = await watchPosition({enableHighAccuracy: true, timeout: 10000, maximumAge: 10000, requestUpdatesInBackground: true}, (position) => {
          console.log('position', position);
        });
        console.log('watchId', watchId);
      }}>Watch Position</button>
      <button onClick={async () => {
        const products = await fetchProducts(['sub_example']);
        console.log('products', products);
      }}>Fetch Products</button>
      <button onClick={async () => {
        const purchaseResponse = await purchaseProduct('sub_example');
        console.log('purchaseResponse', purchaseResponse);
      }}>Purchase Product</button>
      <button onClick={async () => {
        const restorePurchasesResponse = await restorePurchases();
        console.log('restorePurchasesResponse', restorePurchasesResponse);
      }}>Restore Purchases</button>
      <button onClick={async () => {
        const authenticateResponse = await authenticate({authUrl: 'https://bradleat.inkibra.dev/tonetempo-auth-start', callbackScheme: 'nk-tonetempo'});
        console.log('authenticateResponse', authenticateResponse);
      }}>Authenticate</button>
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

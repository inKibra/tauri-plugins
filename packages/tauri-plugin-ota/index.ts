import { invoke } from '@tauri-apps/api/core';

export enum UpdateStatus {
  Ready = 'ready',
  Error = 'error'
}

export type UpdateInfo = {
  update?: string;
  manifest?: {
    version: string;
    url: string;
    hash: string;
    notes?: string;
  };
  error?: string;
}

// Store for registered startup handlers
let registeredStartupHandlers: Array<() => void | Promise<void>> = [];
let updateInfo: UpdateInfo | undefined = undefined;
let prepared = false;
let started = false;

/**
 * Prepare the app by checking for updates and applying them if available
 * @param manifestUrl URL to the update manifest JSON
 * @returns Information about the update status including content if update is available
 */
export async function prepare(manifestUrl: string): Promise<UpdateInfo> {
  if (prepared) {
    throw new Error('App already prepared');
  }
  const result = await invoke<UpdateInfo>('plugin:ota|prepare', { 
    payload: { manifestUrl } 
  });
  updateInfo = result;
  prepared = true;
  return result;
}

/**
 * Start the app by loading the appropriate JavaScript file
 * Using Blob for dynamic loading if update is available
 * @param updateInfo Optional update info from prepare() call
 */
export async function start(): Promise<void> {
  console.info('Starting app...');

  if (!prepared) {
    throw new Error('App not prepared');
  }
  if (started) {
    throw new Error('App already started');
  }
  
  // Check if we have an update
  if (updateInfo?.update) {
    console.info('Loading update from content...');
    
    // Create a blob from the update content
    const blob = new Blob([updateInfo.update], { type: 'application/javascript' });
    const scriptURL = URL.createObjectURL(blob);
    
    // Create script element
    const script = document.createElement('script');
    script.src = scriptURL;
    script.async = true;
    script.defer = true;
    
    // Clean up the URL when the script is loaded
    script.onload = () => URL.revokeObjectURL(scriptURL);
    
    // Append to document
    document.head.appendChild(script);
  } else {
    console.info('Loading default bundled script...');
    
    // Create script element for bundled app.js
    const script = document.createElement('script');
    script.src = './app.js';
    script.async = true;
    script.defer = true;
    
    // Append to document
    document.head.appendChild(script);
  }
}

/**
 * Register a startup handler that will be called when the app starts
 * @param handler Function to call when the app starts
 */
export function register(handler: () => void | Promise<void>): void {
  if (started) {
    throw new Error('App already started');
  }
  // Add to the list of handlers
  registeredStartupHandlers.push(handler);
  
  // Execute immediately if this is called after initial startup
  handler();
}

import { invoke } from '@tauri-apps/api/core';

export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:sharing|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function share(text: string, url: string): Promise<void> {
  return await invoke<void>('plugin:sharing|share', {
    payload: {
      text,
      url,
    },
  });
}

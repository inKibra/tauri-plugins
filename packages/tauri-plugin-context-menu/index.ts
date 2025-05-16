import { invoke } from '@tauri-apps/api/core';

export type MenuItem = {
  title: string
  id: string
}

export async function showContextMenu(items: MenuItem[]): Promise<string | null> {
  return await invoke<{selectedId?: string}>('plugin:context-menu|show_context_menu', {
    payload: {
      items,
    },
  }).then((r) => (r.selectedId ? r.selectedId : null));
}

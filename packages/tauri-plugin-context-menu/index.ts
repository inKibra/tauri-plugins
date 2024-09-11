import { invoke } from '@tauri-apps/api/core'

export type MenuItem = {
  title: string
  id: string
}

export async function showContextMenu(items: MenuItem[], x: number, y: number): Promise<string | null> {
  return await invoke<{selectedId?: string}>('plugin:context-menu|show_context_menu', {
    payload: {
      items,
      x,
      y,
    },
  }).then((r) => (r.selectedId ? r.selectedId : null));
}

import { invoke } from '@tauri-apps/api/core'

export type CustomPattern = {
  durations: number[];
  intensities: number[];
}

export type HapticVibrateRequest = {
  pattern: 'short' | 'medium' | 'long';
} | {
  pattern: 'custom';
  customPattern: CustomPattern;
}

export type HapticResponse = {
  success: boolean;
}

export async function vibrate(pattern: HapticVibrateRequest): Promise<HapticResponse> {
  return await invoke<HapticResponse>('plugin:haptic-feedback|vibrate', {
    payload: pattern
  });
}

export async function impactFeedback(style: 'light' | 'medium' | 'heavy'): Promise<HapticResponse> {
  return await invoke<HapticResponse>('plugin:haptic-feedback|impact_feedback', {
    payload: {
      style,
    },
  });
}

export async function selectionFeedback(): Promise<HapticResponse> {
  return await invoke<HapticResponse>('plugin:haptic-feedback|selection_feedback');
}

import { Channel, invoke } from '@tauri-apps/api/core'

export type NotificationPermissionStatus = {
  status: 'prompt' | 'denied' | 'granted'
}

export type NotificationRegistrationStatus = {
  isRegistered: boolean
  token?: string
}

export type NotificationRegistrationResult = {
  success: boolean
  token?: string
  error?: string
}

export type WatchNotificationResult = {
  success: boolean
}

export type NotificationEventType = 
  | 'BACKGROUND_TAP'
  | 'FOREGROUND_TAP'
  | 'FOREGROUND_DELIVERY'
  | 'BACKGROUND_DELIVERY'

export type NotificationEvent = {
  type: NotificationEventType
  payload: Record<string, string>
}

export async function checkPermissions(): Promise<NotificationPermissionStatus> {
  return await invoke('plugin:notifications|check_permissions')
}

export async function requestPermissions(): Promise<NotificationPermissionStatus> {
  return await invoke('plugin:notifications|request_permissions')
}

export async function checkRegistrationStatus(): Promise<NotificationRegistrationStatus> {
  return await invoke('plugin:notifications|check_registration_status')
}

export async function registerForRemoteNotifications(): Promise<NotificationRegistrationResult> {
  return await invoke('plugin:notifications|register_for_remote_notifications')
}

export async function watchNotifications(
  callback: (event: NotificationEvent) => void
): Promise<WatchNotificationResult> {
  const channel = new Channel<NotificationEvent>();
  
  channel.onmessage = (message) => {
    callback(message);
  };

  return await invoke('plugin:notifications|watch_notifications', {
    channel,
  });
}
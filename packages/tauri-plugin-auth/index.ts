import { invoke } from '@tauri-apps/api/core'

export type AuthenticateArgs = {
  authUrl: string;
  callbackScheme: string;
}

export type AuthResult = {
  success: boolean;
  token?: string;
  error?: string;
}

export async function authenticate(args: AuthenticateArgs): Promise<AuthResult> {
  return await invoke('plugin:auth|authenticate', { payload: args });
}
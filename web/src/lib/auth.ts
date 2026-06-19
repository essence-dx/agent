import {
  getCompatStorageItem,
  removeCompatStorageItem,
  safeLocalStorage,
  setCompatStorageItem,
} from './compatStorage';

const PRIMARY_TOKEN_KEY = 'dx_agents_token';
const LEGACY_TOKEN_KEYS = ['zeroclaw_token'] as const;
export const AUTH_TOKEN_KEYS = [PRIMARY_TOKEN_KEY, ...LEGACY_TOKEN_KEYS] as const;

/**
 * Retrieve the stored authentication token.
 */
export function getToken(): string | null {
  return getCompatStorageItem(safeLocalStorage(), PRIMARY_TOKEN_KEY, LEGACY_TOKEN_KEYS);
}

/**
 * Store an authentication token.
 */
export function setToken(token: string): void {
  setCompatStorageItem(safeLocalStorage(), PRIMARY_TOKEN_KEY, LEGACY_TOKEN_KEYS, token);
}

/**
 * Remove the stored authentication token.
 */
export function clearToken(): void {
  removeCompatStorageItem(safeLocalStorage(), PRIMARY_TOKEN_KEY, LEGACY_TOKEN_KEYS);
}

/**
 * Returns true if a token is currently stored.
 */
export function isAuthenticated(): boolean {
  const token = getToken();
  return token !== null && token.length > 0;
}

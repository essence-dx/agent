export const DASHBOARD_COMPATIBILITY_USAGE_KEY = 'dx_agents_dashboard_compatibility_usage_v1';

export type CompatStorageOperation =
  | 'primary_read'
  | 'legacy_read'
  | 'primary_write'
  | 'legacy_write'
  | 'primary_remove'
  | 'legacy_remove'
  | 'migration';

export interface DashboardCompatibilityUsageCounter {
  primary_key: string;
  legacy_keys: string[];
  scope: 'localStorage' | 'sessionStorage' | 'unknown';
  primary_read_count: number;
  primary_write_count: number;
  primary_remove_count: number;
  legacy_read_count: number;
  legacy_write_count: number;
  legacy_remove_count: number;
  migration_count: number;
  last_operation: CompatStorageOperation;
  last_seen_ms: number;
}

export interface DashboardCompatibilityUsageTelemetry {
  schema_version: 'dx.dashboard_compatibility_usage.v1';
  storage_key: typeof DASHBOARD_COMPATIBILITY_USAGE_KEY;
  updated_at_ms: number;
  primary_usage_count: number;
  legacy_usage_count: number;
  legacy_read_count: number;
  legacy_write_count: number;
  legacy_remove_count: number;
  migration_count: number;
  counters: DashboardCompatibilityUsageCounter[];
}

export function safeLocalStorage(): Storage | null {
  try {
    return window.localStorage;
  } catch {
    return null;
  }
}

export function safeSessionStorage(): Storage | null {
  try {
    return window.sessionStorage;
  } catch {
    return null;
  }
}

export function getCompatStorageItem(
  storage: Storage | null | undefined,
  primaryKey: string,
  legacyKeys: readonly string[] = [],
): string | null {
  if (!storage) return null;
  try {
    const primaryValue = storage.getItem(primaryKey);
    if (primaryValue !== null) {
      recordCompatStorageUse(storage, primaryKey, legacyKeys, 'primary_read');
      return primaryValue;
    }
    for (const legacyKey of legacyKeys) {
      const legacyValue = storage.getItem(legacyKey);
      if (legacyValue !== null) {
        recordCompatStorageUse(storage, primaryKey, legacyKeys, 'legacy_read');
        storage.setItem(primaryKey, legacyValue);
        recordCompatStorageUse(storage, primaryKey, legacyKeys, 'migration');
        return legacyValue;
      }
    }
  } catch {
    return null;
  }
  return null;
}

export function setCompatStorageItem(
  storage: Storage | null | undefined,
  primaryKey: string,
  legacyKeys: readonly string[],
  value: string,
): void {
  if (!storage) return;
  try {
    storage.setItem(primaryKey, value);
    recordCompatStorageUse(storage, primaryKey, legacyKeys, 'primary_write');
    for (const legacyKey of legacyKeys) {
      storage.setItem(legacyKey, value);
      recordCompatStorageUse(storage, primaryKey, legacyKeys, 'legacy_write');
    }
  } catch {
    // Storage can be unavailable in restricted browser modes.
  }
}

export function removeCompatStorageItem(
  storage: Storage | null | undefined,
  primaryKey: string,
  legacyKeys: readonly string[] = [],
): void {
  if (!storage) return;
  try {
    storage.removeItem(primaryKey);
    recordCompatStorageUse(storage, primaryKey, legacyKeys, 'primary_remove');
    for (const legacyKey of legacyKeys) {
      storage.removeItem(legacyKey);
      recordCompatStorageUse(storage, primaryKey, legacyKeys, 'legacy_remove');
    }
  } catch {
    // Storage can be unavailable in restricted browser modes.
  }
}

export function getDashboardCompatibilityUsageTelemetry(
  storage: Storage | null | undefined = safeLocalStorage(),
): DashboardCompatibilityUsageTelemetry {
  const telemetry = readUsageTelemetry(storage);
  return telemetry ?? emptyUsageTelemetry();
}

export function resetDashboardCompatibilityUsageTelemetry(
  storage: Storage | null | undefined = safeLocalStorage(),
): void {
  if (!storage) return;
  try {
    storage.removeItem(DASHBOARD_COMPATIBILITY_USAGE_KEY);
  } catch {
    // Storage can be unavailable in restricted browser modes.
  }
}

function recordCompatStorageUse(
  storage: Storage,
  primaryKey: string,
  legacyKeys: readonly string[],
  operation: CompatStorageOperation,
): void {
  if (primaryKey === DASHBOARD_COMPATIBILITY_USAGE_KEY) return;

  const telemetry = readUsageTelemetry(storage) ?? emptyUsageTelemetry();
  const normalizedPrimary = normalizeCompatStorageKey(primaryKey);
  const normalizedLegacy = legacyKeys.map(normalizeCompatStorageKey);
  const counterKey = `${storageScope(storage)}:${normalizedPrimary}`;
  const existing = telemetry.counters.find((counter) => `${counter.scope}:${counter.primary_key}` === counterKey);
  const counter: DashboardCompatibilityUsageCounter = existing ?? {
    primary_key: normalizedPrimary,
    legacy_keys: normalizedLegacy,
    scope: storageScope(storage),
    primary_read_count: 0,
    primary_write_count: 0,
    primary_remove_count: 0,
    legacy_read_count: 0,
    legacy_write_count: 0,
    legacy_remove_count: 0,
    migration_count: 0,
    last_operation: operation,
    last_seen_ms: Date.now(),
  };

  counter.legacy_keys = Array.from(new Set([...counter.legacy_keys, ...normalizedLegacy]));
  counter.last_operation = operation;
  counter.last_seen_ms = Date.now();

  switch (operation) {
    case 'primary_read':
      counter.primary_read_count += 1;
      telemetry.primary_usage_count += 1;
      break;
    case 'primary_write':
      counter.primary_write_count += 1;
      telemetry.primary_usage_count += 1;
      break;
    case 'primary_remove':
      counter.primary_remove_count += 1;
      telemetry.primary_usage_count += 1;
      break;
    case 'legacy_read':
      counter.legacy_read_count += 1;
      telemetry.legacy_read_count += 1;
      telemetry.legacy_usage_count += 1;
      break;
    case 'legacy_write':
      counter.legacy_write_count += 1;
      telemetry.legacy_write_count += 1;
      telemetry.legacy_usage_count += 1;
      break;
    case 'legacy_remove':
      counter.legacy_remove_count += 1;
      telemetry.legacy_remove_count += 1;
      telemetry.legacy_usage_count += 1;
      break;
    case 'migration':
      counter.migration_count += 1;
      telemetry.migration_count += 1;
      break;
  }

  if (!existing) telemetry.counters.push(counter);
  telemetry.updated_at_ms = Date.now();
  writeUsageTelemetry(storage, telemetry);
}

function readUsageTelemetry(storage: Storage | null | undefined): DashboardCompatibilityUsageTelemetry | null {
  if (!storage) return null;
  try {
    const raw = storage.getItem(DASHBOARD_COMPATIBILITY_USAGE_KEY);
    if (!raw) return null;
    const parsed = JSON.parse(raw) as Partial<DashboardCompatibilityUsageTelemetry>;
    if (parsed.schema_version !== 'dx.dashboard_compatibility_usage.v1') return null;
    return {
      ...emptyUsageTelemetry(),
      ...parsed,
      counters: Array.isArray(parsed.counters) ? parsed.counters : [],
    };
  } catch {
    return null;
  }
}

function writeUsageTelemetry(storage: Storage, telemetry: DashboardCompatibilityUsageTelemetry): void {
  try {
    storage.setItem(DASHBOARD_COMPATIBILITY_USAGE_KEY, JSON.stringify(telemetry));
  } catch {
    // Telemetry is best-effort and intentionally redacted.
  }
}

function emptyUsageTelemetry(): DashboardCompatibilityUsageTelemetry {
  return {
    schema_version: 'dx.dashboard_compatibility_usage.v1',
    storage_key: DASHBOARD_COMPATIBILITY_USAGE_KEY,
    updated_at_ms: Date.now(),
    primary_usage_count: 0,
    legacy_usage_count: 0,
    legacy_read_count: 0,
    legacy_write_count: 0,
    legacy_remove_count: 0,
    migration_count: 0,
    counters: [],
  };
}

function normalizeCompatStorageKey(key: string): string {
  if (key.startsWith('dx_agents_chat_history_v1:')) return 'dx_agents_chat_history_v1:';
  if (key.startsWith('zeroclaw_chat_history_v1:')) return 'zeroclaw_chat_history_v1:';
  return key;
}

function storageScope(storage: Storage): 'localStorage' | 'sessionStorage' | 'unknown' {
  try {
    if (storage === window.localStorage) return 'localStorage';
    if (storage === window.sessionStorage) return 'sessionStorage';
  } catch {
    return 'unknown';
  }
  return 'unknown';
}

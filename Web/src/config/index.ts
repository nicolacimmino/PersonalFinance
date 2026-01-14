/**
 * Application configuration
 *
 * Centralizes all environment variables and configuration settings.
 * This provides a single source of truth for configuration across the application.
 */

/**
 * Application configuration object
 */
export const config = {
  /**
   * API backend host URL
   */
  apiHost: import.meta.env.VITE_HOST as string,

  /**
   * Current environment name (dev, staging, prod, etc.)
   */
  envName: import.meta.env.VITE_ENV as string,

  /**
   * LocalStorage key for API authentication
   */
  apiKeyStorageKey: 'pfinanceApiKey',

  /**
   * LocalStorage key for privacy mode preference
   */
  privacyStorageKey: 'privacy',

  /**
   * LocalStorage key for compact mode preference
   */
  compactStorageKey: 'compact',

  /**
   * LocalStorage key for reference currency preference
   */
  refCurrencyStorageKey: 'refCurrency',

  /**
   * LocalStorage key for selected year filter
   */
  yearStorageKey: 'year',
} as const;

/**
 * Helper function to get API key from localStorage
 */
export function getApiKey(): string | null {
  return localStorage.getItem(config.apiKeyStorageKey);
}

/**
 * Helper function to set API key in localStorage
 */
export function setApiKey(apiKey: string): void {
  localStorage.setItem(config.apiKeyStorageKey, apiKey);
}

/**
 * Helper function to get privacy setting from localStorage
 */
export function getPrivacySetting(): boolean {
  return localStorage.getItem(config.privacyStorageKey) === 'true';
}

/**
 * Helper function to set privacy setting in localStorage
 */
export function setPrivacySetting(privacy: boolean): void {
  localStorage.setItem(config.privacyStorageKey, String(privacy));
}

/**
 * Helper function to get year filter from localStorage
 */
export function getYearFilter(): number | null {
  const year = localStorage.getItem(config.yearStorageKey);
  return year ? parseInt(year, 10) : null;
}

/**
 * Helper function to set year filter in localStorage
 */
export function setYearFilter(year: number): void {
  localStorage.setItem(config.yearStorageKey, String(year));
}

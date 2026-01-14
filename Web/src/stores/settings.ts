import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { config } from '@/config';

/**
 * Settings Store
 *
 * Manages user preferences and settings with localStorage persistence.
 * Centralizes settings that were previously scattered across components.
 */
export const useSettingsStore = defineStore('settings', () => {
  // State
  const apiKey = ref<string>(localStorage.getItem(config.apiKeyStorageKey) || '');
  const privacy = ref<boolean>(localStorage.getItem(config.privacyStorageKey) === 'true');
  const compact = ref<boolean>(localStorage.getItem(config.compactStorageKey) === 'true');
  const refCurrency = ref<boolean>(localStorage.getItem(config.refCurrencyStorageKey) === 'true');
  const year = ref<number>(
    parseInt(localStorage.getItem(config.yearStorageKey) || String(new Date().getFullYear()), 10)
  );

  // Getters
  const hasApiKey = computed(() => apiKey.value !== '');
  const isPrivacyEnabled = computed(() => privacy.value);
  const isCompactMode = computed(() => compact.value);
  const isRefCurrencyEnabled = computed(() => refCurrency.value);
  const selectedYear = computed(() => year.value);

  // Actions
  function setApiKey(newApiKey: string) {
    apiKey.value = newApiKey;
    localStorage.setItem(config.apiKeyStorageKey, newApiKey);
  }

  function clearApiKey() {
    apiKey.value = '';
    localStorage.removeItem(config.apiKeyStorageKey);
  }

  function togglePrivacy() {
    privacy.value = !privacy.value;
    localStorage.setItem(config.privacyStorageKey, String(privacy.value));
  }

  function setPrivacy(value: boolean) {
    privacy.value = value;
    localStorage.setItem(config.privacyStorageKey, String(value));
  }

  function toggleCompact() {
    compact.value = !compact.value;
    localStorage.setItem(config.compactStorageKey, String(compact.value));
  }

  function setCompact(value: boolean) {
    compact.value = value;
    localStorage.setItem(config.compactStorageKey, String(value));
  }

  function toggleRefCurrency() {
    refCurrency.value = !refCurrency.value;
    localStorage.setItem(config.refCurrencyStorageKey, String(refCurrency.value));
  }

  function setRefCurrency(value: boolean) {
    refCurrency.value = value;
    localStorage.setItem(config.refCurrencyStorageKey, String(value));
  }

  function setYear(newYear: number) {
    year.value = newYear;
    localStorage.setItem(config.yearStorageKey, String(newYear));
  }

  return {
    // State
    apiKey,
    privacy,
    compact,
    refCurrency,
    year,
    // Getters
    hasApiKey,
    isPrivacyEnabled,
    isCompactMode,
    isRefCurrencyEnabled,
    selectedYear,
    // Actions
    setApiKey,
    clearApiKey,
    togglePrivacy,
    setPrivacy,
    toggleCompact,
    setCompact,
    toggleRefCurrency,
    setRefCurrency,
    setYear,
  };
});

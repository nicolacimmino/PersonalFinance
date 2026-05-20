import { ref, computed } from 'vue'
import { config } from '@/config'

type Theme = 'light' | 'dark' | null

// Shared reactive state (singleton pattern)
const apiKey = ref(localStorage.getItem(config.apiKeyStorageKey) || '')
const privacy = ref(localStorage.getItem(config.privacyStorageKey) === 'true')
const compact = ref(localStorage.getItem(config.compactStorageKey) === 'true')
const refCurrency = ref(localStorage.getItem(config.refCurrencyStorageKey) === 'true')
const year = ref(
  parseInt(localStorage.getItem(config.yearStorageKey) || String(new Date().getFullYear()), 10)
)
const theme = ref<Theme>(localStorage.getItem(config.themeStorageKey) as Theme)
const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches
const isDark = computed(() => {
  if (theme.value === 'dark') return true
  if (theme.value === 'light') return false
  return systemDark
})

function applyThemeClass(value: Theme) {
  document.documentElement.classList.remove('theme-light', 'theme-dark')
  if (value === 'light') document.documentElement.classList.add('theme-light')
  if (value === 'dark') document.documentElement.classList.add('theme-dark')
}

applyThemeClass(theme.value)

export function useSettings() {
  function setApiKey(value: string) {
    apiKey.value = value
    localStorage.setItem(config.apiKeyStorageKey, value)
  }

  function clearApiKey() {
    apiKey.value = ''
    localStorage.removeItem(config.apiKeyStorageKey)
  }

  function setPrivacy(value: boolean) {
    privacy.value = value
    localStorage.setItem(config.privacyStorageKey, String(value))
  }

  function togglePrivacy() {
    setPrivacy(!privacy.value)
  }

  function setCompact(value: boolean) {
    compact.value = value
    localStorage.setItem(config.compactStorageKey, String(value))
  }

  function toggleCompact() {
    setCompact(!compact.value)
  }

  function setRefCurrency(value: boolean) {
    refCurrency.value = value
    localStorage.setItem(config.refCurrencyStorageKey, String(value))
  }

  function toggleRefCurrency() {
    setRefCurrency(!refCurrency.value)
  }

  function setYear(value: number) {
    year.value = value
    localStorage.setItem(config.yearStorageKey, String(value))
  }

  function setTheme(value: Theme) {
    theme.value = value
    if (value === null) {
      localStorage.removeItem(config.themeStorageKey)
    } else {
      localStorage.setItem(config.themeStorageKey, value)
    }
    applyThemeClass(value)
  }

  function toggleTheme() {
    setTheme(isDark.value ? 'light' : 'dark')
  }

  return {
    // State
    apiKey,
    privacy,
    compact,
    refCurrency,
    year,
    // Computed
    hasApiKey: () => apiKey.value !== '',
    // Actions
    setApiKey,
    clearApiKey,
    setPrivacy,
    togglePrivacy,
    setCompact,
    toggleCompact,
    setRefCurrency,
    toggleRefCurrency,
    setYear,
    theme,
    isDark,
    setTheme,
    toggleTheme,
  }
}

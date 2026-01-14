import { computed } from 'vue';
import { useSettingsStore } from '@/stores/settings';

/**
 * Privacy Composable
 *
 * Manages privacy mode state across the application.
 * Used by components to toggle visibility of sensitive financial data.
 *
 * Usage:
 *   const { privacy, togglePrivacy } = usePrivacy();
 */
export function usePrivacy() {
  const settingsStore = useSettingsStore();

  // Reactive privacy state from store
  const privacy = computed({
    get: () => settingsStore.privacy,
    set: (value: boolean) => settingsStore.setPrivacy(value),
  });

  // Toggle privacy mode
  function togglePrivacy() {
    settingsStore.togglePrivacy();
  }

  // Set privacy mode explicitly
  function setPrivacy(value: boolean) {
    settingsStore.setPrivacy(value);
  }

  return {
    privacy,
    togglePrivacy,
    setPrivacy,
  };
}

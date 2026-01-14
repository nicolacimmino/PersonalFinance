import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Alert } from '@/types';
import { getAlerts } from '@/services/api';
import { useSettingsStore } from './settings';

/**
 * Alerts Store
 *
 * Manages system alerts and notifications.
 */
export const useAlertsStore = defineStore('alerts', () => {
  // State
  const alerts = ref<Alert[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Getters
  const alertsByItemType = computed(() => {
    return alerts.value.reduce((acc, alert) => {
      (acc[alert.item] = acc[alert.item] || []).push(alert);
      return acc;
    }, {} as Record<string, Alert[]>);
  });

  const hasAlerts = computed(() => alerts.value.length > 0);

  const alertCount = computed(() => alerts.value.length);

  // Actions
  async function fetchAlerts() {
    const settingsStore = useSettingsStore();

    if (!settingsStore.hasApiKey) {
      error.value = 'API key required';
      return;
    }

    loading.value = true;
    error.value = null;

    try {
      const response = await getAlerts();
      alerts.value = response;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load alerts';
      console.error('Failed to load alerts:', err);
    } finally {
      loading.value = false;
    }
  }

  function clearAlerts() {
    alerts.value = [];
    error.value = null;
  }

  return {
    // State
    alerts,
    loading,
    error,
    // Getters
    alertsByItemType,
    hasAlerts,
    alertCount,
    // Actions
    fetchAlerts,
    clearAlerts,
  };
});

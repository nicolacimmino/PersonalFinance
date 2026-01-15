import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Indicator } from '@/types';
import { getIndicators } from '@/services/api';
import { useSettingsStore } from './settings';

/**
 * Indicators Store
 *
 * Manages financial indicators and metrics.
 */
export const useIndicatorsStore = defineStore('indicators', () => {
  // State
  const indicators = ref<Indicator[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Getters
  const indicatorsByLabel = computed(() => {
    return indicators.value.reduce((acc, indicator) => {
      acc[indicator.label] = indicator;
      return acc;
    }, {} as Record<string, Indicator>);
  });

  const hasIndicators = computed(() => indicators.value.length > 0);

  // Helper to get value of a specific indicator
  const getIndicatorValue = computed(() => (label: string) => {
    const indicator = indicators.value.find(ind => ind.label === label);
    return indicator ? indicator.totalCents : 0;
  });

  // Actions
  async function fetchIndicators(year?: number) {
    const settingsStore = useSettingsStore();
    const selectedYear = year ?? settingsStore.year;

    loading.value = true;
    error.value = null;

    try {
      const response = await getIndicators(selectedYear);
      // Sort indicators by position
      indicators.value = response.indicators.sort((a, b) =>
        labelToPosition(a.label) > labelToPosition(b.label) ? 1 : -1
      );
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load indicators';
      console.error('Failed to load indicators:', err);
    } finally {
      loading.value = false;
    }
  }

  function clearIndicators() {
    indicators.value = [];
    error.value = null;
  }

  // Helper function to determine indicator position
  function labelToPosition(type: string): number {
    switch (type.substring(0, 4)) {
      case 'CASH':
        return 0;
      case 'INVT':
        return 1;
      case 'TONW':
        return 3;
      case 'INAT':
        return 5;
      case 'INPS':
        return 6;
      case 'EXPE':
        return 7;
      case 'CFAT':
        return 8;
      case 'CFOA':
        return 9;
      default:
        if (type.substring(0, 1) === 'C') {
          return 10000000 - (getIndicatorValue.value(type) / 100);
        }
        return 9;
    }
  }

  return {
    // State
    indicators,
    loading,
    error,
    // Getters
    indicatorsByLabel,
    hasIndicators,
    getIndicatorValue,
    // Actions
    fetchIndicators,
    clearIndicators,
  };
});

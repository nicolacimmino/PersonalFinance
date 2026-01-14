import { computed } from 'vue';
import { useSettingsStore } from '@/stores/settings';

/**
 * Year Filter Composable
 *
 * Manages year selection for filtering transactions and reports.
 *
 * Usage:
 *   const { selectedYear, setYear } = useYearFilter();
 */
export function useYearFilter() {
  const settingsStore = useSettingsStore();

  // Reactive year from store
  const selectedYear = computed({
    get: () => settingsStore.year,
    set: (value: number) => settingsStore.setYear(value),
  });

  // Set year explicitly
  function setYear(year: number) {
    settingsStore.setYear(year);
  }

  // Get current year
  function getCurrentYear(): number {
    return new Date().getFullYear();
  }

  // Generate list of available years (from start year to current)
  function getAvailableYears(startYear = 2024): number[] {
    const currentYear = getCurrentYear();
    const years: number[] = [];

    for (let year = startYear; year <= currentYear; year++) {
      years.push(year);
    }

    return years;
  }

  return {
    selectedYear,
    setYear,
    getCurrentYear,
    getAvailableYears,
  };
}

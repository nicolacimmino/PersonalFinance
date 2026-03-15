import { useSettings } from './useSettings'

export function useYearFilter() {
  const settings = useSettings()

  function getCurrentYear(): number {
    return new Date().getFullYear()
  }

  function getAvailableYears(startYear = 2024): number[] {
    const currentYear = getCurrentYear()
    const years: number[] = []
    for (let year = startYear; year <= currentYear; year++) {
      years.push(year)
    }
    return years
  }

  return {
    selectedYear: settings.year,
    setYear: settings.setYear,
    getCurrentYear,
    getAvailableYears
  }
}

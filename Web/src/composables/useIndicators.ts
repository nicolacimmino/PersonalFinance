import { useQuery } from '@tanstack/vue-query'
import { getIndicators } from '@/services/api'
import type { Indicator } from '@/types'
import { computed, type Ref } from 'vue'

export function useIndicators(year: Ref<number>) {
  const { data, isLoading, error } = useQuery<{ indicators: Indicator[] }>({
    queryKey: ['indicators', year],
    queryFn: () => getIndicators(year.value)
  })

  const indicators = computed(() => {
    if (!data.value?.indicators) return []
    return data.value.indicators.sort((a, b) =>
      labelToPosition(a.label, getIndicatorValue) > labelToPosition(b.label, getIndicatorValue) ? 1 : -1
    )
  })

  function getIndicatorValue(label: string): number {
    const indicator = indicators.value.find(ind => ind.label === label)
    return indicator ? indicator.totalCents : 0
  }

  return {
    indicators,
    getIndicatorValue,
    isLoading,
    error
  }
}

function labelToPosition(type: string, getIndicatorValue: (label: string) => number): number {
  switch (type.substring(0, 4)) {
    case 'CASH':
      return 0
    case 'INVT':
      return 1
    case 'TONW':
      return 3
    case 'INAT':
      return 5
    case 'INPS':
      return 6
    case 'EXPE':
      return 7
    case 'CFAT':
      return 8
    case 'CFOA':
      return 9
    default:
      if (type.substring(0, 1) === 'C') {
        return 10000000 - (getIndicatorValue(type) / 100)
      }
      return 9
  }
}

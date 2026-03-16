import { useQuery } from '@tanstack/vue-query'
import { getBudgets } from '@/services/api'
import type { Budget } from '@/types'
import { computed } from 'vue'

export function useBudgets() {
  const { data, isLoading, error } = useQuery<Budget[]>({
    queryKey: ['budgets'],
    queryFn: getBudgets
  })

  const budgets = computed(() => data.value ?? [])

  return {
    budgets,
    isLoading,
    error
  }
}

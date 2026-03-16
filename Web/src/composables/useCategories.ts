import { useQuery } from '@tanstack/vue-query'
import { getCategories } from '@/services/api'
import type { Category } from '@/types'
import { computed } from 'vue'

export function useCategories() {
  const { data, isLoading, error } = useQuery<Category[]>({
    queryKey: ['categories'],
    queryFn: getCategories
  })

  const categories = computed(() => data.value ?? [])

  return {
    categories,
    isLoading,
    error
  }
}

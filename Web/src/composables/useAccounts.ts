
import { useQuery } from '@tanstack/vue-query'
import { getAccounts, getAccount } from '@/services/api'
import type { Account } from '@/types'
import { computed } from 'vue'

export function useAccounts() {
  const { data, isLoading, error } = useQuery<Account[]>({
    queryKey: ['accounts'],
    queryFn: getAccounts
  })

  const accounts = computed(() => data.value ?? [])

  return {
    accounts,
    isLoading,
    error
  }
}

export function useAccount(id: string | number) {
  return useQuery<Account>({
    queryKey: ['account', id],
    queryFn: () => getAccount(id)
  })
}

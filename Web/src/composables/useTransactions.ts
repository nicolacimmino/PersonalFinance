import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
import {
  getTransactions,
  getTransaction,
  updateTransaction,
  createTransaction
} from '@/services/api'
import type { Transaction } from '@/types'
import { computed, type Ref } from 'vue'

export function useTransactions(
  accountId: Ref<string | undefined>,
  category: Ref<string | undefined>,
  year: Ref<number>
) {
  const queryClient = useQueryClient()

  const { data, isLoading, error } = useQuery<Transaction[]>({
    queryKey: ['transactions', accountId, category, year],
    queryFn: () => getTransactions(accountId.value, category.value, year.value)
  })

  const transactions = computed(() => data.value ?? [])

  const updateMutation = useMutation({
    mutationFn: (params: {
      id: string | number
      category: string
      type: string
      description: string
      accountTo: string | number
    }) => updateTransaction(params.id, params.category, params.type, params.description, params.accountTo),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['transactions'] })
    }
  })

  const createMutation = useMutation({
    mutationFn: (transaction: Partial<Transaction>) => createTransaction(transaction),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['transactions'] })
    }
  })

  return {
    transactions,
    isLoading,
    error,
    updateTransaction: updateMutation.mutateAsync,
    createTransaction: createMutation.mutateAsync,
    isUpdating: updateMutation.isPending,
    isCreating: createMutation.isPending
  }
}

export function useTransaction(id: string | number) {
  return useQuery<Transaction>({
    queryKey: ['transaction', id],
    queryFn: () => getTransaction(id)
  })
}

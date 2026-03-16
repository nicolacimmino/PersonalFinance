import { useQuery } from '@tanstack/vue-query'
import { getAlerts } from '@/services/api'
import type { Alert } from '@/types'
import { computed } from 'vue'

export function useAlerts() {
  const { data, isLoading, error } = useQuery<Alert[]>({
    queryKey: ['alerts'],
    queryFn: getAlerts
  })

  const alerts = computed(() => data.value ?? [])

  const alertsByItemType = computed(() => {
    return alerts.value.reduce((acc, alert) => {
      (acc[alert.item] = acc[alert.item] || []).push(alert)
      return acc
    }, {} as Record<string, Alert[]>)
  })

  return {
    alerts,
    alertsByItemType,
    isLoading,
    error
  }
}

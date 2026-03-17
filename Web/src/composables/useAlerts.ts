import { useQuery } from '@tanstack/vue-query'
import { getAlerts } from '@/services/api'
import type { Alert } from '@/types'
import { computed } from 'vue'
import { useSettings } from './useSettings'

export function useAlerts() {
  const { apiKey } = useSettings()

  const { data, isLoading, error } = useQuery<Alert[]>({
    queryKey: ['alerts'],
    queryFn: getAlerts,
    enabled: computed(() => apiKey.value !== '')
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

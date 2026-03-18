import httpClient from './httpClient';
import type { Alert } from '@/types';

/**
 * Alert Service
 *
 * Handles all alert-related API calls.
 */

/**
 * Get all alerts
 */
export async function getAlerts(): Promise<Alert[]> {
  const response = await httpClient.get<Alert[]>('/api/v2/alerts/');
  return response;
}

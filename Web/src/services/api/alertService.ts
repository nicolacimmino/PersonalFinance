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
  const response = await httpClient.get<Alert[]>('/api/alerts/');
  return response;
}

/**
 * Get a single alert by ID (if API supports it)
 */
export async function getAlert(id: string | number): Promise<Alert> {
  const response = await httpClient.get<Alert>(`/api/alerts/${id}`);
  return response;
}

/**
 * Mark alert as read/resolved (if API supports it)
 */
export async function markAlertAsRead(id: string | number): Promise<Alert> {
  const response = await httpClient.patch<Alert>(`/api/alerts/${id}`, {
    status: 'read',
  });
  return response;
}

/**
 * Delete an alert (if API supports it)
 */
export async function deleteAlert(id: string | number): Promise<void> {
  await httpClient.delete(`/api/alerts/${id}`);
}

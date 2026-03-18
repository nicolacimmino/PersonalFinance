import httpClient from './httpClient';
import type { Budget } from '@/types';

/**
 * Budget Service
 *
 * Handles all budget-related API calls.
 */

/**
 * Get all budgets
 */
export async function getBudgets(): Promise<Budget[]> {
  const response = await httpClient.get<Budget[]>('/api/v2/budgets/');
  return response;
}

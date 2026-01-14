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
  const response = await httpClient.get<Budget[]>('/api/budgets/');
  return response;
}

/**
 * Get a single budget by ID (if API supports it)
 */
export async function getBudget(id: string | number): Promise<Budget> {
  const response = await httpClient.get<Budget>(`/api/budgets/${id}`);
  return response;
}

/**
 * Create a new budget (if API supports it)
 */
export async function createBudget(budget: Partial<Budget>): Promise<Budget> {
  const response = await httpClient.post<Budget>('/api/budgets/', budget);
  return response;
}

/**
 * Update a budget (if API supports it)
 */
export async function updateBudget(
  id: string | number,
  budget: Partial<Budget>
): Promise<Budget> {
  const response = await httpClient.patch<Budget>(`/api/budgets/${id}`, budget);
  return response;
}

/**
 * Delete a budget (if API supports it)
 */
export async function deleteBudget(id: string | number): Promise<void> {
  await httpClient.delete(`/api/budgets/${id}`);
}

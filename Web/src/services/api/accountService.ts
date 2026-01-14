import httpClient from './httpClient';
import type { Account } from '@/types';

/**
 * Account Service
 *
 * Handles all account-related API calls.
 */

/**
 * Get all accounts
 */
export async function getAccounts(): Promise<Account[]> {
  const response = await httpClient.get<Account[]>('/api/accounts/');
  return response;
}

/**
 * Get a single account by ID
 */
export async function getAccount(id: string | number): Promise<Account> {
  const response = await httpClient.get<Account>(`/api/accounts/${id}`);
  return response;
}

/**
 * Create a new account (if API supports it)
 */
export async function createAccount(account: Partial<Account>): Promise<Account> {
  const response = await httpClient.post<Account>('/api/accounts/', account);
  return response;
}

/**
 * Update an account (if API supports it)
 */
export async function updateAccount(
  id: string | number,
  account: Partial<Account>
): Promise<Account> {
  const response = await httpClient.patch<Account>(`/api/accounts/${id}`, account);
  return response;
}

/**
 * Delete an account (if API supports it)
 */
export async function deleteAccount(id: string | number): Promise<void> {
  await httpClient.delete(`/api/accounts/${id}`);
}

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
  const response = await httpClient.get<Account[]>('/api/v2/accounts/');
  return response;
}

/**
 * Get a single account by ID
 */
export async function getAccount(id: string | number): Promise<Account> {
  const response = await httpClient.get<Account>(`/api/v2/accounts/${id}`);
  return response;
}

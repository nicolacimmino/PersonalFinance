import httpClient from './httpClient';
import type { Transaction } from '@/types';
import moment from 'moment';

/**
 * Transaction Service
 *
 * Handles all transaction-related API calls.
 */

/**
 * Get transactions with optional filters
 */
export async function getTransactions(
  accountId?: string,
  category?: string,
  year?: number
): Promise<Transaction[]> {
  const selectedYear = year ?? moment().year();
  const dateFrom = `${selectedYear}-01-01`;
  const dateTo = `${selectedYear}-12-31`;

  const response = await httpClient.get<Transaction[]>('/api/transactions/', {
    params: {
      category: category || '',
      account: accountId || '',
      date_from: dateFrom,
      date_to: dateTo,
    },
  });

  return response;
}

/**
 * Get a single transaction by ID
 */
export async function getTransaction(id: string | number): Promise<Transaction> {
  const response = await httpClient.get<Transaction>(`/api/transactions/${id}`);
  return response;
}

/**
 * Update transaction category, type, and description
 */
export async function updateTransactionCategory(
  id: string | number,
  category: string,
  type: string,
  description: string
): Promise<Transaction> {
  const response = await httpClient.patch<Transaction>(`/api/transactions/${id}`, {
    type,
    category,
    description,
  });
  return response;
}

/**
 * Update transaction account_to (for transfers)
 */
export async function updateTransactionAccountTo(
  id: string | number,
  accountTo: string | number
): Promise<Transaction> {
  const response = await httpClient.patch<Transaction>(`/api/transactions/${id}`, {
    type: 'TRANSFER',
    account_to: accountTo,
  });
  return response;
}

/**
 * Create a new transaction
 */
export async function createTransaction(
  transaction: Partial<Transaction>
): Promise<Transaction | string> {
  try {
    const response = await httpClient.post<Transaction>('/api/transactions', {
      type: transaction.type,
      account_id: transaction.account?.id,
      booking_date: transaction.booking_date,
      category: transaction.category,
      creditor_name: transaction.creditor_name,
      description: transaction.description,
      amount_cents: transaction.amount_cents
        ? Math.round(transaction.amount_cents)
        : undefined,
      account_to: transaction.destination_account?.id || null,
    });
    return response;
  } catch (error) {
    console.error('Failed to create transaction:', error);
    return 'ERROR';
  }
}

/**
 * Delete a transaction (if API supports it)
 */
export async function deleteTransaction(id: string | number): Promise<void> {
  await httpClient.delete(`/api/transactions/${id}`);
}

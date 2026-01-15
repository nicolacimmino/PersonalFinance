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

  const response = await httpClient.get<Transaction[]>('/api/v2/transactions/', {
    params: {
      category: category || '',
      account: accountId || '',
      dateFrom: dateFrom,
      dateTo: dateTo,
    },
  });

  return response;
}

/**
 * Get a single transaction by ID
 */
export async function getTransaction(id: string | number): Promise<Transaction> {
  const response = await httpClient.get<Transaction>(`/api/v2/transactions/${id}`);
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
  const response = await httpClient.patch<Transaction>(`/api/v2/transactions/${id}`, {
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
  const response = await httpClient.patch<Transaction>(`/api/v2/transactions/${id}`, {
    type: 'TRANSFER',
    accountTo: accountTo,
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
    const response = await httpClient.post<Transaction>('/api/v2/transactions', {
      type: transaction.type,
      accountId: transaction.account?.id,
      bookingDate: transaction.bookingDate,
      category: transaction.category,
      creditorName: transaction.creditorName,
      description: transaction.description,
      amountCents: transaction.amountCents
        ? Math.round(transaction.amountCents)
        : undefined,
      accountTo: transaction.destinationAccount?.id || null,
    });
    return response;
  } catch (error) {
    console.error('Failed to create transaction:', error);
    return 'ERROR';
  }
}

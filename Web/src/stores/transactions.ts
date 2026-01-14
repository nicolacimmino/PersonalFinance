import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Transaction, TransactionsByDate } from '@/types';
import {
  getTransactions,
  getTransaction,
  updateTransactionCategory,
  updateTransactionAccountTo,
  createTransaction as createTransactionService,
} from '@/services/api';
import { useSettingsStore } from './settings';

/**
 * Transactions Store
 *
 * Manages financial transactions data and operations.
 */
export const useTransactionsStore = defineStore('transactions', () => {
  // State
  const transactions = ref<Transaction[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Getters
  const transactionsByDate = computed<TransactionsByDate>(() => {
    return transactions.value.reduce((acc, transaction) => {
      const date = transaction.booking_date;
      if (!acc[date]) {
        acc[date] = [];
      }
      acc[date].push(transaction);
      return acc;
    }, {} as TransactionsByDate);
  });

  const transactionsById = computed(() => {
    return transactions.value.reduce((acc, transaction) => {
      acc[transaction.id] = transaction;
      return acc;
    }, {} as Record<string | number, Transaction>);
  });

  const hasTransactions = computed(() => transactions.value.length > 0);

  // Actions
  async function fetchTransactions(
    accountId?: string,
    category?: string,
    year?: number
  ) {
    const settingsStore = useSettingsStore();
    const selectedYear = year ?? settingsStore.year;

    loading.value = true;
    error.value = null;

    try {
      const response = await getTransactions(
        accountId,
        category,
        selectedYear
      );
      transactions.value = response;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load transactions';
      console.error('Failed to load transactions:', err);
    } finally {
      loading.value = false;
    }
  }

  async function fetchTransaction(id: string | number) {
    loading.value = true;
    error.value = null;

    try {
      const response = await getTransaction(id);
      // Update in list if exists, otherwise add
      const index = transactions.value.findIndex(t => t.id === id);
      if (index >= 0) {
        transactions.value[index] = response;
      } else {
        transactions.value.push(response);
      }
      return response;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load transaction';
      console.error('Failed to load transaction:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function updateTransaction(
    id: string | number,
    category: string,
    type: string,
    description: string
  ) {
    loading.value = true;
    error.value = null;

    try {
      const response = await updateTransactionCategory(
        id,
        category,
        type,
        description
      );
      // Update in list
      const index = transactions.value.findIndex(t => t.id === id);
      if (index >= 0) {
        transactions.value[index] = response;
      }
      return response;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to update transaction';
      console.error('Failed to update transaction:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function updateTransactionAccountTo(
    id: string | number,
    accountTo: string | number
  ) {
    loading.value = true;
    error.value = null;

    try {
      const response = await updateTransactionAccountTo(id, accountTo);
      // Update in list
      const index = transactions.value.findIndex(t => t.id === id);
      if (index >= 0) {
        transactions.value[index] = response;
      }
      return response;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to update transaction';
      console.error('Failed to update transaction:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function createTransaction(transaction: Partial<Transaction>) {
    loading.value = true;
    error.value = null;

    try {
      const response = await createTransactionService(transaction);
      if (response !== 'ERROR') {
        // Add to list if successful and is a Transaction object
        if (typeof response !== 'string') {
          transactions.value.push(response);
        }
      }
      return response;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to create transaction';
      console.error('Failed to create transaction:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  function clearTransactions() {
    transactions.value = [];
    error.value = null;
  }

  return {
    // State
    transactions,
    loading,
    error,
    // Getters
    transactionsByDate,
    transactionsById,
    hasTransactions,
    // Actions
    fetchTransactions,
    fetchTransaction,
    updateTransaction,
    updateTransactionAccountTo,
    createTransaction,
    clearTransactions,
  };
});

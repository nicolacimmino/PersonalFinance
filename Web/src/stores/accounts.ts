import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Account } from '@/types';
import { getAccounts, getAccount } from '@/services/api';

/**
 * Accounts Store
 *
 * Manages financial accounts data.
 */
export const useAccountsStore = defineStore('accounts', () => {
  // State
  const accounts = ref<Account[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Getters
  const accountsById = computed(() => {
    return accounts.value.reduce((acc, account) => {
      acc[account.id] = account;
      return acc;
    }, {} as Record<string | number, Account>);
  });

  const hasAccounts = computed(() => accounts.value.length > 0);

  // Actions
  async function fetchAccounts() {
    loading.value = true;
    error.value = null;

    try {
      const response = await getAccounts();
      accounts.value = response;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load accounts';
      console.error('Failed to load accounts:', err);
    } finally {
      loading.value = false;
    }
  }

  async function fetchAccount(id: string | number) {
    loading.value = true;
    error.value = null;

    try {
      const response = await getAccount(id);
      // Update in list if exists, otherwise add
      const index = accounts.value.findIndex(acc => acc.id === id);
      if (index >= 0) {
        accounts.value[index] = response;
      } else {
        accounts.value.push(response);
      }
      return response;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load account';
      console.error('Failed to load account:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  function clearAccounts() {
    accounts.value = [];
    error.value = null;
  }

  return {
    // State
    accounts,
    loading,
    error,
    // Getters
    accountsById,
    hasAccounts,
    // Actions
    fetchAccounts,
    fetchAccount,
    clearAccounts,
  };
});

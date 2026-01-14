import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Budget } from '@/types';
import { getBudgets } from '@/services/api';

/**
 * Budgets Store
 *
 * Manages budget data and tracking.
 */
export const useBudgetsStore = defineStore('budgets', () => {
  // State
  const budgets = ref<Budget[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Getters
  const budgetsById = computed(() => {
    return budgets.value.reduce((acc, budget) => {
      acc[budget.id] = budget;
      return acc;
    }, {} as Record<string | number, Budget>);
  });

  const hasBudgets = computed(() => budgets.value.length > 0);

  // Actions
  async function fetchBudgets() {
    loading.value = true;
    error.value = null;

    try {
      const response = await getBudgets();
      budgets.value = response;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load budgets';
      console.error('Failed to load budgets:', err);
    } finally {
      loading.value = false;
    }
  }

  function clearBudgets() {
    budgets.value = [];
    error.value = null;
  }

  return {
    // State
    budgets,
    loading,
    error,
    // Getters
    budgetsById,
    hasBudgets,
    // Actions
    fetchBudgets,
    clearBudgets,
  };
});

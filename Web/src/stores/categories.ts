import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Category } from '@/types';
import { getCategories } from '@/services/api';

/**
 * Categories Store
 *
 * Manages transaction categories.
 */
export const useCategoriesStore = defineStore('categories', () => {
  // State
  const categories = ref<Category[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Getters
  const categoriesByName = computed(() => {
    return categories.value.reduce((acc, category) => {
      acc[category.name] = category;
      return acc;
    }, {} as Record<string, Category>);
  });

  const hasCategories = computed(() => categories.value.length > 0);

  // Actions
  async function fetchCategories() {
    loading.value = true;
    error.value = null;

    try {
      const response = await getCategories();
      categories.value = response;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load categories';
      console.error('Failed to load categories:', err);
    } finally {
      loading.value = false;
    }
  }

  function clearCategories() {
    categories.value = [];
    error.value = null;
  }

  return {
    // State
    categories,
    loading,
    error,
    // Getters
    categoriesByName,
    hasCategories,
    // Actions
    fetchCategories,
    clearCategories,
  };
});

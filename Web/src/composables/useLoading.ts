import { ref, computed } from 'vue';

/**
 * Loading State Composable
 *
 * Manages loading states for async operations.
 * Supports multiple concurrent loading operations with counter.
 *
 * Usage:
 *   const { loading, isLoading, startLoading, stopLoading, withLoading } = useLoading();
 *
 * Example with withLoading:
 *   await withLoading(async () => {
 *     await fetchData();
 *   });
 */
export function useLoading(initialState = false) {
  const loadingCounter = ref(initialState ? 1 : 0);

  // Computed property for easier template usage
  const isLoading = computed(() => loadingCounter.value > 0);

  // Legacy support - direct boolean value
  const loading = computed({
    get: () => loadingCounter.value > 0,
    set: (value: boolean) => {
      loadingCounter.value = value ? 1 : 0;
    },
  });

  function startLoading() {
    loadingCounter.value++;
  }

  function stopLoading() {
    if (loadingCounter.value > 0) {
      loadingCounter.value--;
    }
  }

  function resetLoading() {
    loadingCounter.value = 0;
  }

  /**
   * Wraps an async function with loading state management
   */
  async function withLoading<T>(fn: () => Promise<T>): Promise<T> {
    startLoading();
    try {
      return await fn();
    } finally {
      stopLoading();
    }
  }

  return {
    loading,
    isLoading,
    loadingCounter,
    startLoading,
    stopLoading,
    resetLoading,
    withLoading,
  };
}

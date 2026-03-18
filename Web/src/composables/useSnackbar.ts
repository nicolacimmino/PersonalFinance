import { ref } from 'vue';

/**
 * Snackbar/Toast Composable
 *
 * Manages snackbar notifications in a reactive way.
 * Replaces direct DOM manipulation with Vue reactive state.
 *
 * Usage:
 *   const { message, isVisible, showSnackbar, hideSnackbar } = useSnackbar();
 *
 * Example:
 *   showSnackbar('Transaction saved successfully');
 */
export function useSnackbar() {
  const message = ref('');
  const isVisible = ref(false);
  const autoHideTimeout = ref<number | null>(null);

  /**
   * Show snackbar with a message
   * @param msg - Message to display
   * @param duration - Auto-hide duration in ms (default: 3000, 0 = no auto-hide)
   */
  function showSnackbar(msg: string, duration = 3000) {
    // Clear any existing timeout
    if (autoHideTimeout.value) {
      clearTimeout(autoHideTimeout.value);
      autoHideTimeout.value = null;
    }

    message.value = msg;
    isVisible.value = true;

    // Auto-hide after duration
    if (duration > 0) {
      autoHideTimeout.value = window.setTimeout(() => {
        hideSnackbar();
      }, duration);
    }
  }

  /**
   * Hide snackbar
   */
  function hideSnackbar() {
    isVisible.value = false;

    // Clear timeout if exists
    if (autoHideTimeout.value) {
      clearTimeout(autoHideTimeout.value);
      autoHideTimeout.value = null;
    }

    // Clear message after animation (300ms)
    setTimeout(() => {
      message.value = '';
    }, 300);
  }

  return {
    message,
    isVisible,
    showSnackbar,
    hideSnackbar,
  };
}

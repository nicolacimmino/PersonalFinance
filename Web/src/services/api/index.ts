/**
 * API Services Barrel Export
 *
 * Centralized export for all API service modules.
 * Import services from this file for cleaner imports.
 *
 * Example:
 *   import { getTransactions, getAccounts } from '@/services/api';
 */

// Export HTTP client
export { default as httpClient } from './httpClient';

// Transaction Service
export {
  getTransactions,
  getTransaction,
  updateTransactionCategory,
  updateTransactionAccountTo,
  createTransaction,
} from './transactionService';

// Account Service
export {
  getAccounts,
  getAccount,
} from './accountService';

// Category Service
export {
  getCategories,
} from './categoryService';

// Budget Service
export {
  getBudgets,
} from './budgetService';

// Report Service
export {
  getCategoryReport,
  getIndicators,
} from './reportService';

// Alert Service
export {
  getAlerts,
} from './alertService';

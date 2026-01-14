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
  deleteTransaction,
} from './transactionService';

// Account Service
export {
  getAccounts,
  getAccount,
  createAccount,
  updateAccount,
  deleteAccount,
} from './accountService';

// Category Service
export {
  getCategories,
  getCategory,
  createCategory,
  updateCategory,
  deleteCategory,
} from './categoryService';

// Budget Service
export {
  getBudgets,
  getBudget,
  createBudget,
  updateBudget,
  deleteBudget,
} from './budgetService';

// Report Service
export {
  getCategoryReport,
  getIndicators,
} from './reportService';

// Alert Service
export {
  getAlerts,
  getAlert,
  markAlertAsRead,
  deleteAlert,
} from './alertService';

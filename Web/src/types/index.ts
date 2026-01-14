/**
 * Core domain type definitions for Personal Finance application
 */

/**
 * Transaction types
 */
export type TransactionType = 'EXPENSE' | 'INCOME' | 'TRANSFER';

/**
 * Alert severity levels
 */
export type AlertLevel = 'INFO' | 'WARNING' | 'ERROR';

/**
 * Transaction interface representing a financial transaction
 */
export interface Transaction {
  id: string | number;
  type: TransactionType;
  account_name: string;
  account_to_name?: string;
  category?: string;
  creditor_name?: string;
  description: string;
  amount_cents: number;
  currency: string;
  amount_cents_in_ref_currency?: number;
  ref_currency?: string;
  booking_date: string;
  account?: Account;
  account_id?: string | number;
  account_to?: string | number;
  destination_account?: Account;
}

/**
 * Account interface representing a financial account
 */
export interface Account {
  id: string | number;
  name: string;
  currency?: string;
  balance_cents?: number;
  type?: string;
}

/**
 * Category interface for transaction categorization
 */
export interface Category {
  id?: string | number;
  name: string;
  parent?: string;
  type?: TransactionType;
}

/**
 * Budget interface for budget tracking
 */
export interface Budget {
  id: string | number;
  description: string;
  from_date: string;
  to_date: string;
  amount_cents: number;
  spent_cents: number;
  currency: string;
  transactions: number;
  category?: string;
}

/**
 * Indicator interface for financial indicators
 */
export interface Indicator {
  label: string;
  total_cents: number;
  description?: string;
}

/**
 * Alert interface for system alerts and notifications
 */
export interface Alert {
  category: string;
  message: string;
  item: string;
  item_id: string;
  level: AlertLevel;
}

/**
 * Category report entry for spending analysis
 */
export interface CategoryReportEntry {
  category: string;
  total_cents: number;
  transaction_count?: number;
  percentage?: number;
  subcategories?: CategoryReportEntry[];
}

/**
 * Date-grouped transactions for display
 */
export interface TransactionsByDate {
  [date: string]: Transaction[];
}

/**
 * User settings interface
 */
export interface UserSettings {
  apiKey: string;
  privacy: boolean;
  compact: boolean;
  refCurrency: string;
  year: number;
}

/**
 * API response wrapper (if needed)
 */
export interface ApiResponse<T> {
  data: T;
  status: number;
  message?: string;
}

/**
 * Chart data structure for reporting
 */
export interface ChartData {
  labels: string[];
  datasets: {
    label: string;
    data: number[];
    backgroundColor?: string[];
    borderColor?: string[];
    borderWidth?: number;
  }[];
}

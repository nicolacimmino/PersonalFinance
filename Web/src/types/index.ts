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
  accountName: string;
  accountToName?: string;
  category?: string;
  creditorName?: string;
  description: string;
  amountCents: number;
  currency: string;
  amountCentsInRefCurrency?: number;
  refCurrency?: string;
  bookingDate: string;
  account?: Account;
  accountId?: string | number;
  accountTo?: string | number;
  destinationAccount?: Account;
}

/**
 * Account interface representing a financial account
 */
export interface Account {
  id: string | number
  name: string
  description: string
  currency?: string
  refCurrency?: string
  balanceCents?: number
  balanceRefCurrencyCents?: number
  type?: string
  canCreateTransactions: boolean
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
  fromDate: string;
  toDate: string;
  amountCents: number;
  spentCents: number;
  currency: string;
  transactions: number;
  category?: string;
}

/**
 * Indicator interface for financial indicators
 */
export interface Indicator {
  label: string;
  totalCents: number;
  description?: string;
}

/**
 * Alert interface for system alerts and notifications
 */
export interface Alert {
  category: string;
  message: string;
  item: string;
  itemId: string;
  level: AlertLevel;
}

/**
 * Category report entry for spending analysis
 */
export interface CategoryReportEntry {
  category: string;
  totalCents: number;
  transactionCount?: number;
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

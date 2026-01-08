/**
 * Database model for application.alerts
 */
export interface Alert {
  category: string; // BALANCE, CATEGORY_MISSING, CATEGORY_INVALID, ERROR, OVERSPENT
  message: string;
  item: string; // BUDGETS, TRANSACTIONS, ACCOUNTS
  item_id: string;
  level: string; // WARN or ERROR
}

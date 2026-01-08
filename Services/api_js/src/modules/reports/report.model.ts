/**
 * Database model for report by category query result
 */
export interface ReportByCategoryEntry {
  category: string;
  category_type: string;
  amount_cents_eur: number;
  transactions_count: number;
  category_description: string;
}

/**
 * Database model for indicator
 */
export interface Indicator {
  label: string;
  amount_cents: number;
}

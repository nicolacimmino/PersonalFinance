/**
 * Database model for raw.transactions
 */
export interface Transaction {
  id: number;
  type_: string;
  account_id: number;
  amount_cents: number;
  category: string;
  creditor_name: string;
  description: string;
  booking_date: Date;
  value_date: Date;
  account_to: number | null;
}

/**
 * Database model for application.transactions (enriched view)
 */
export interface ApplicationTransaction {
  id: number;
  transaction_type: string;
  account_id: number;
  amount_cents: number;
  category: string;
  creditor_name: string;
  description: string;
  booking_date: Date;
  value_date: Date;
  account_to: number | null;
  amount_cents_eur: number;
  account_name: string;
  currency: string;
  account_type: string;
  account_to_name: string | null;
  receipt_id: number | null;
}

/**
 * Model for creating a new transaction
 */
export interface NewTransaction {
  type_: string;
  account_id: number;
  amount_cents: number;
  category: string;
  creditor_name: string;
  description: string;
  booking_date: Date;
  value_date: Date;
  account_to: number | null;
}

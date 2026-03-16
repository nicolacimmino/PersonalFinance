// Database model for ob_accounts table
export interface ObAccount {
  id: string; // UUID
  provider: string;
  provider_account_id: string;
  name: string;
  req_status: string;
  last_sync: Date | null;
  account_id: number;
}

// Insert model for ob_transactions table
export interface NewObTransaction {
  ob_account_id: string; // UUID
  transaction_id: string;
  booking_date: string;
  value_date: string;
  booking_date_time: string;
  transaction_amount_cents: number;
  transaction_amount_currency: string;
  creditor_name: string;
  debtor_name: string;
  debtor_account_iban: string;
  remittance_information_unstructured: string;
  balance_after_transaction_amount_cents: number;
  balance_after_transaction_currency: string;
  balance_after_transaction_type: string;
  internal_transaction_id: string;
}

// Database model for accounts table
export interface Account {
  id: number;
  code: string;
  description: string;
  currency: string;
  pri_transactions_src: string;
  status: string;
  iban: string;
}

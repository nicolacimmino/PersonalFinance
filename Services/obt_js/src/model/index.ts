// Database model for accounts table
export interface Account {
  id: number;
  code: string;
  description: string;
  currency: string;
}

// Database model for ob_accounts table
export interface ObAccount {
  id: string; // UUID
  provider: string;
  provider_account_id: string;
  name: string;
  account_id: number;
}

// Database model for ob_transactions table
export interface ObTransaction {
  id: string; // UUID
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

// Database model for transactions table
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
}

// Insert model for new transactions
export interface NewTransaction {
  type_: string;
  account_id: number;
  amount_cents: number;
  category: string;
  creditor_name: string;
  description: string;
  booking_date: Date;
  value_date: Date;
}

// Combined result from the JOIN query
export interface ObTransactionWithAccount {
  ob_transaction: ObTransaction;
  ob_account: ObAccount;
  account: Account;
}

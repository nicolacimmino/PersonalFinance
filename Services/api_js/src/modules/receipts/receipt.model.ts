/**
 * Database model for application.receipts
 */
export interface ApplicationReceipt {
  id: number;
  date: Date;
  amount_cents: number;
  currency: string;
  ext_id: string;
  merchant_name: string;
  merchant_address: string;
  scan_file_name: string;
  transaction_id: number | null;
  transaction_category: string | null;
  transaction_amount_cents: number | null;
  transaction_currency: string | null;
  account_code: string | null;
  account_description: string | null;
}

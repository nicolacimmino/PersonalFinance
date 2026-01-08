/**
 * Data Transfer Object for Transaction responses
 */
export interface TransactionDto {
  id: number;
  type: string;
  account_id: number;
  account_name: string;
  booking_date: string;
  category: string;
  creditor_name: string;
  description: string;
  amount_cents: number;
  currency: string;
  amount_cents_in_ref_currency: number;
  ref_currency: string;
  account_to: number | null;
  account_to_name: string | null;
  receipt_id: number | null;
}

/**
 * DTO for PATCH /transactions/:id
 */
export interface PatchTransactionDto {
  category?: string;
  description?: string;
  account_to?: number;
  type?: string;
}

/**
 * DTO for POST /transactions
 */
export interface CreateTransactionDto {
  amount_cents: number;
  category?: string;
  description?: string;
  creditor_name?: string;
  account_id: number;
  account_to?: number;
  type?: string;
  booking_date?: string;
  value_date?: string;
}

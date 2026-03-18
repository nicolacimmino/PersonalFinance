/**
 * V2 Data Transfer Object for Transaction responses (camelCase)
 */
export interface TransactionDtoV2 {
  id: number;
  type: string;
  accountId: number;
  accountName: string;
  bookingDate: string;
  category: string;
  creditorName: string;
  description: string;
  amountCents: number;
  currency: string;
  amountCentsInRefCurrency: number;
  refCurrency: string;
  accountTo: number | null;
  accountToName: string | null;
  receiptId: number | null;
}

/**
 * DTO for PATCH /api/v2/transactions/:id
 */
export interface PatchTransactionDtoV2 {
  category?: string;
  description?: string;
  accountTo?: number;
  type?: string;
}

/**
 * DTO for POST /api/v2/transactions
 */
export interface CreateTransactionDtoV2 {
  amountCents: number;
  category?: string;
  description?: string;
  creditorName?: string;
  accountId: number;
  accountTo?: number;
  type?: string;
  bookingDate?: string;
  valueDate?: string;
}

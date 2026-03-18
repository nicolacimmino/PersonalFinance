/**
 * V2 Data Transfer Object for Receipt responses (camelCase)
 */
export interface ReceiptDtoV2 {
  id: number;
  date: string;
  amountCents: number;
  currency: string;
  externalId: string;
  merchantName: string;
  merchantAddress: string;
  scanFileName: string;
  transactionId: number | null;
  transactionCategory: string | null;
  transactionAmountCents: number | null;
  transactionCurrency: string | null;
  accountCode: string | null;
  accountDescription: string | null;
}

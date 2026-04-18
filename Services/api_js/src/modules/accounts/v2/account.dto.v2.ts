/**
 * V2 Data Transfer Object for Account responses (camelCase)
 */
export interface AccountDtoV2 {
  id: number;
  code: string;
  description: string;
  currency: string;
  refCurrency: string;
  iban: string;
  status: string;
  assetType: string;
  primaryTransactionSource: string;
  canCreateTransactions: boolean;
  balanceCents: number;
  balanceRefCurrencyCents: number;
}

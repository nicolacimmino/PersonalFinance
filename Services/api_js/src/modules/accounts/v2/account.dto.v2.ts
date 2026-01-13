/**
 * V2 Data Transfer Object for Account responses (camelCase)
 */
export interface AccountDtoV2 {
  id: number;
  code: string;
  description: string;
  currency: string;
  iban: string;
  status: string;
  assetType: string;
  primaryTransactionSource: string;
  balanceCents: number;
  balanceEurCents: number;
}

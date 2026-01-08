/**
 * Data Transfer Object for Account responses
 */
export interface AccountDto {
  id: number;
  code: string;
  description: string;
  currency: string;
  iban: string;
  status: string;
  asset_type: string;
  pri_transactions_src: string;
  balance_cents: number;
  balance_eur_cents: number;
}

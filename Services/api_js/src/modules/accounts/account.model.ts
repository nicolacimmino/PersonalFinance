/**
 * Database model for application.accounts
 */
export interface ApplicationAccount {
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

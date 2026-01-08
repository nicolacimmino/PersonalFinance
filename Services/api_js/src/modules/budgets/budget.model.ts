/**
 * Database model for application.budgets
 */
export interface ApplicationBudgets {
  id: number;
  from_date: Date;
  to_date: Date;
  code: string;
  description: string;
  active: boolean;
  currency: string;
  amount_cents: number;
  spent_cents_eur: number;
  spent_cents: number;
  transactions: number;
}

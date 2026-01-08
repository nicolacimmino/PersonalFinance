/**
 * Data Transfer Object for Budget responses
 */
export interface BudgetOverviewDto {
  id: number;
  from_date: string;
  to_date: string;
  code: string;
  description: string;
  active: boolean;
  currency: string;
  amount_cents: number;
  spent_cents_eur: number;
  spent_cents: number;
  transactions: number;
}

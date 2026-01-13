/**
 * V2 Data Transfer Object for Budget responses (camelCase)
 */
export interface BudgetOverviewDtoV2 {
  id: number;
  fromDate: string;
  toDate: string;
  code: string;
  description: string;
  active: boolean;
  currency: string;
  amountCents: number;
  spentCentsEur: number;
  spentCents: number;
  transactionCount: number;
}

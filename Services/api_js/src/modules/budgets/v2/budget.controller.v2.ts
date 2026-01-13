import { Request, Response } from 'express';
import * as service from '../budget.service';
import { BudgetOverviewDtoV2 } from './budget.dto.v2';
import { ApplicationBudgets } from '../budget.model';
import { formatDate } from '../../../utils/dateHelpers';

/**
 * Transform database model to V2 DTO (camelCase)
 */
function toBudgetDtoV2(budget: ApplicationBudgets): BudgetOverviewDtoV2 {
  return {
    id: budget.id,
    fromDate: formatDate(new Date(budget.from_date)),
    toDate: formatDate(new Date(budget.to_date)),
    code: budget.code,
    description: budget.description,
    active: budget.active,
    currency: budget.currency,
    amountCents: budget.amount_cents,
    spentCentsEur: budget.spent_cents_eur,
    spentCents: budget.spent_cents,
    transactionCount: budget.transactions,
  };
}

/**
 * GET /api/v2/budgets
 * Get all budgets
 */
export async function getBudgets(_req: Request, res: Response): Promise<void> {
  const budgets = await service.getBudgets();
  const dtos = budgets.map(toBudgetDtoV2);
  res.json(dtos);
}

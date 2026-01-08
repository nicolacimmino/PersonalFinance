import { Request, Response } from 'express';
import * as service from './budget.service';
import { BudgetOverviewDto } from './budget.dto';
import { ApplicationBudgets } from './budget.model';
import { formatDate } from '../../utils/dateHelpers';

/**
 * Transform database model to DTO
 */
function toBudgetDto(budget: ApplicationBudgets): BudgetOverviewDto {
  return {
    id: budget.id,
    from_date: formatDate(new Date(budget.from_date)),
    to_date: formatDate(new Date(budget.to_date)),
    code: budget.code,
    description: budget.description,
    active: budget.active,
    currency: budget.currency,
    amount_cents: budget.amount_cents,
    spent_cents_eur: budget.spent_cents_eur,
    spent_cents: budget.spent_cents,
    transactions: budget.transactions,
  };
}

/**
 * GET /api/budgets
 * Get all budgets
 */
export async function getBudgets(_req: Request, res: Response): Promise<void> {
  const budgets = await service.getBudgets();
  const dtos = budgets.map(toBudgetDto);
  res.json(dtos);
}

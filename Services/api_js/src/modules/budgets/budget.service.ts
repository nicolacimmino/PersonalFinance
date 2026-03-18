import { query } from '../../config/database';
import { ApplicationBudgets } from './budget.model';

/**
 * Get all budgets from the database
 * @returns Array of budgets ordered by ID DESC
 */
export async function getBudgets(): Promise<ApplicationBudgets[]> {
  const sql = `
    SELECT id, from_date, to_date, code, description, active,
           currency, amount_cents, spent_cents_eur, spent_cents, transactions
    FROM application.budgets
    ORDER BY id DESC
  `;

  const result = await query<ApplicationBudgets>(sql);
  return result.rows;
}

import { query } from '../../config/database';
import { ApplicationAccount } from './account.model';
import { AppError } from '../../middleware/errorHandler';

/**
 * Get all accounts from the database
 * @returns Array of accounts
 */
export async function getAccounts(): Promise<ApplicationAccount[]> {
  const sql = `
    SELECT id, code, description, currency, iban, status,
           asset_type, pri_transactions_src, balance_cents, balance_eur_cents
    FROM application.accounts
  `;

  const result = await query<ApplicationAccount>(sql);
  return result.rows;
}

/**
 * Get a single account by ID
 * @param id Account ID
 * @returns Account data
 * @throws AppError if account not found
 */
export async function getAccount(id: number): Promise<ApplicationAccount> {
  const sql = `
    SELECT id, code, description, currency, iban, status,
           asset_type, pri_transactions_src, balance_cents, balance_eur_cents
    FROM application.accounts
    WHERE id = $1
  `;

  const result = await query<ApplicationAccount>(sql, [id]);

  if (result.rows.length === 0) {
    throw new AppError(404, `Account with id ${id} not found`);
  }

  return result.rows[0];
}

import { query } from '../../config/database';
import { ApplicationReceipt } from './receipt.model';
import { AppError } from '../../middleware/errorHandler';

/**
 * Get all receipts from the database
 * @returns Array of receipts
 */
export async function getReceipts(): Promise<ApplicationReceipt[]> {
  const sql = `
    SELECT id, date, amount_cents, currency, ext_id,
           merchant_name, merchant_address, scan_file_name,
           transaction_id, transaction_category, transaction_amount_cents,
           transaction_currency, account_code, account_description
    FROM application.receipts
  `;

  const result = await query<ApplicationReceipt>(sql);
  return result.rows;
}

/**
 * Get a single receipt by ID
 * @param id Receipt ID
 * @returns Receipt data
 * @throws AppError if receipt not found
 */
export async function getReceipt(id: number): Promise<ApplicationReceipt> {
  const sql = `
    SELECT id, date, amount_cents, currency, ext_id,
           merchant_name, merchant_address, scan_file_name,
           transaction_id, transaction_category, transaction_amount_cents,
           transaction_currency, account_code, account_description
    FROM application.receipts
    WHERE id = $1
  `;

  const result = await query<ApplicationReceipt>(sql, [id]);

  if (result.rows.length === 0) {
    throw new AppError(404, `Receipt with id ${id} not found`);
  }

  return result.rows[0];
}

import { query } from '../../config/database';
import { Alert } from './alert.model';

/**
 * Get all alerts from the database
 * @returns Array of alerts ordered by item
 */
export async function getAlerts(): Promise<Alert[]> {
  const sql = `
    SELECT category, message, item, item_id, level
    FROM application.alerts
    ORDER BY item
  `;

  const result = await query<Alert>(sql);
  return result.rows;
}

import { query } from '../../config/database';
import { ApplicationCategory } from './category.model';

/**
 * Get all categories from the database
 * @returns Array of categories ordered by code
 */
export async function getCategories(): Promise<ApplicationCategory[]> {
  const sql = `
    SELECT id, code, color, discontinued
    FROM application.categories
    ORDER BY code
  `;

  const result = await query<ApplicationCategory>(sql);
  return result.rows;
}

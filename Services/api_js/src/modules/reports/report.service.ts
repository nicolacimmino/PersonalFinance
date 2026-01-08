import { query } from '../../config/database';
import { ReportByCategoryEntry, Indicator } from './report.model';

/**
 * Get report by category with date range
 * @param dateFrom Start date (YYYY-MM-DD)
 * @param dateTo End date (YYYY-MM-DD)
 * @returns Array of report entries grouped by category
 */
export async function getReportByCategory(
  dateFrom: string,
  dateTo: string
): Promise<ReportByCategoryEntry[]> {
  const sql = `
    SELECT category,
           c.type                              as category_type,
           CAST(count(*) AS int4)              as transactions_count,
           CAST(sum(amount_cents_eur) as int4) as amount_cents_eur,
           c.description as category_description
    FROM application.transactions t
    INNER JOIN raw.categories c ON c.code = t.category
    WHERE t.category_type <> 'TRANSFER'
      AND t.category_type <> 'INITIAL'
      AND (booking_date BETWEEN $1 AND $2)
    GROUP BY category, c.type, c.description
  `;

  const result = await query<ReportByCategoryEntry>(sql, [
    `${dateFrom} 00:00:00`,
    `${dateTo} 00:00:00`,
  ]);

  return result.rows;
}

/**
 * Get KPI indicators by calling the stored function
 * @param dateFrom Start date (YYYY-MM-DD)
 * @param dateTo End date (YYYY-MM-DD)
 * @returns Array of indicators
 */
export async function getKpis(
  dateFrom: string,
  dateTo: string
): Promise<Indicator[]> {
  const sql = `
    SELECT * FROM application.get_indicators($1, $2)
  `;

  const result = await query<Indicator>(sql, [
    `${dateFrom} 00:00:00`,
    `${dateTo} 00:00:00`,
  ]);

  return result.rows;
}

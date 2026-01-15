import httpClient from './httpClient';
import type { CategoryReportEntry, Indicator } from '@/types';
import moment from 'moment';

/**
 * Report Service
 *
 * Handles all reporting and analytics API calls.
 */

/**
 * Get category spending/income report
 */
export async function getCategoryReport(year?: number): Promise<CategoryReportEntry[]> {
  const selectedYear = year ?? moment().year();
  const dateFrom = `${selectedYear}-01-01`;
  const dateTo = `${selectedYear}-12-31`;

  const response = await httpClient.get<CategoryReportEntry[]>('/api/v2/reports/by_category/', {
    params: {
      dateFrom: dateFrom,
      dateTo: dateTo,
    },
  });

  return response;
}

/**
 * Get financial indicators
 */
export async function getIndicators(year?: number): Promise<Indicator[]> {
  const selectedYear = year ?? moment().year();
  const dateFrom = `${selectedYear}-01-01`;
  const dateTo = `${selectedYear}-12-31`;

  const response = await httpClient.get<Indicator[]>('/api/v2/reports/indicators/', {
    params: {
      dateFrom: dateFrom,
      dateTo: dateTo,
    },
  });

  return response;
}

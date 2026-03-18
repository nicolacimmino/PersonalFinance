import { Request, Response } from 'express';
import * as service from '../report.service';
import {
  ReportByCategoryDtoV2,
  ReportByCategoryEntryDtoV2,
  IndicatorReportDtoV2,
  IndicatorDtoV2,
} from './report.dto.v2';
import { ReportByCategoryEntry, Indicator } from '../report.model';
import { parseDateRange } from '../../../utils/dateHelpers';

/**
 * Transform database model to V2 DTO for report by category (camelCase)
 */
function toReportByCategoryEntryDtoV2(
  entry: ReportByCategoryEntry
): ReportByCategoryEntryDtoV2 {
  return {
    category: entry.category,
    type: entry.category_type,
    currency: 'EUR',
    totalCents: entry.amount_cents_eur,
    transactionCount: entry.transactions_count,
    categoryDescription: entry.category_description,
  };
}

/**
 * Transform indicator model to V2 DTO (camelCase)
 */
function toIndicatorDtoV2(indicator: Indicator): IndicatorDtoV2 {
  return {
    label: indicator.label,
    totalCents: indicator.amount_cents,
    currency: 'EUR',
  };
}

/**
 * GET /api/v2/reports/by_category?dateFrom&dateTo
 * Get spending report grouped by category
 */
export async function getReportByCategory(
  req: Request,
  res: Response
): Promise<void> {
  // Accept both camelCase and snake_case for backward compatibility
  const { dateFrom, date_from, dateTo, date_to } = req.query;

  const dateFromParam = dateFrom || date_from;
  const dateToParam = dateTo || date_to;

  const { dateFrom: parsedDateFrom, dateTo: parsedDateTo } = parseDateRange(
    dateFromParam ? String(dateFromParam) : undefined,
    dateToParam ? String(dateToParam) : undefined
  );

  const reports = await service.getReportByCategory(parsedDateFrom, parsedDateTo);
  const entries = reports.map(toReportByCategoryEntryDtoV2);

  const dto: ReportByCategoryDtoV2 = {
    dateFrom: parsedDateFrom,
    dateTo: parsedDateTo,
    reports: entries,
  };

  res.json(dto);
}

/**
 * GET /api/v2/reports/indicators?dateFrom&dateTo
 * Get KPI indicators
 */
export async function getIndicators(
  req: Request,
  res: Response
): Promise<void> {
  // Accept both camelCase and snake_case for backward compatibility
  const { dateFrom, date_from, dateTo, date_to } = req.query;

  const dateFromParam = dateFrom || date_from;
  const dateToParam = dateTo || date_to;

  const { dateFrom: parsedDateFrom, dateTo: parsedDateTo } = parseDateRange(
    dateFromParam ? String(dateFromParam) : undefined,
    dateToParam ? String(dateToParam) : undefined
  );

  const indicators = await service.getKpis(parsedDateFrom, parsedDateTo);
  const dtos = indicators.map(toIndicatorDtoV2);

  const dto: IndicatorReportDtoV2 = {
    indicators: dtos,
    refCurrency: 'EUR',
  };

  res.json(dto);
}

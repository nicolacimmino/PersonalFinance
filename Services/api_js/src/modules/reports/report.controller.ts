import { Request, Response } from 'express';
import * as service from './report.service';
import {
  ReportByCategoryDto,
  ReportByCategoryEntryDto,
  IndicatorReportDto,
  IndicatorDto,
} from './report.dto';
import { ReportByCategoryEntry, Indicator } from './report.model';
import { parseDateRange } from '../../utils/dateHelpers';

/**
 * Transform database model to DTO for report by category
 */
function toReportByCategoryEntryDto(
  entry: ReportByCategoryEntry
): ReportByCategoryEntryDto {
  return {
    category: entry.category,
    type: entry.category_type,
    currency: 'EUR',
    total_cents: entry.amount_cents_eur,
    transactions_count: entry.transactions_count,
    category_description: entry.category_description,
  };
}

/**
 * Transform indicator model to DTO
 */
function toIndicatorDto(indicator: Indicator): IndicatorDto {
  return {
    label: indicator.label,
    total_cents: indicator.amount_cents,
    currency: "EUR"
  };
}

/**
 * GET /api/reports/by_category?date_from&date_to
 * Get spending report grouped by category
 */
export async function getReportByCategory(
  req: Request,
  res: Response
): Promise<void> {
  const { date_from, date_to } = req.query;
  const { dateFrom, dateTo } = parseDateRange(
    date_from ? String(date_from) : undefined,
    date_to ? String(date_to) : undefined
  );

  const reports = await service.getReportByCategory(dateFrom, dateTo);
  const entries = reports.map(toReportByCategoryEntryDto);

  const dto: ReportByCategoryDto = {
    date_from: dateFrom,
    date_to: dateTo,
    reports: entries,
  };

  res.json(dto);
}

/**
 * GET /api/reports/indicators?date_from&date_to
 * Get KPI indicators
 */
export async function getIndicators(
  req: Request,
  res: Response
): Promise<void> {
  const { date_from, date_to } = req.query;
  const { dateFrom, dateTo } = parseDateRange(
    date_from ? String(date_from) : undefined,
    date_to ? String(date_to) : undefined
  );

  const indicators = await service.getKpis(dateFrom, dateTo);
  const dtos = indicators.map(toIndicatorDto);

  const dto: IndicatorReportDto = {
    indicators: dtos,
    ref_currency: 'EUR',
  };

  res.json(dto);
}

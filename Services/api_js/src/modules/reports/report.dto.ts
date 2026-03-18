/**
 * Data Transfer Object for report by category response
 */
export interface ReportByCategoryEntryDto {
  category: string;
  type: string;
  currency: string;
  total_cents: number;
  transactions_count: number;
  category_description: string;
}

export interface ReportByCategoryDto {
  date_from: string;
  date_to: string;
  reports: ReportByCategoryEntryDto[];
}

/**
 * Data Transfer Object for indicator
 */
export interface IndicatorDto {
  label: string;
  total_cents: number;
  currency: string;
}

export interface IndicatorReportDto {
  indicators: IndicatorDto[];
  ref_currency: string;
}

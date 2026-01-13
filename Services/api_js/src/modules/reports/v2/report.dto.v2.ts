/**
 * V2 Data Transfer Object for report by category response (camelCase)
 */
export interface ReportByCategoryEntryDtoV2 {
  category: string;
  type: string;
  currency: string;
  totalCents: number;
  transactionCount: number;
  categoryDescription: string;
}

export interface ReportByCategoryDtoV2 {
  dateFrom: string;
  dateTo: string;
  reports: ReportByCategoryEntryDtoV2[];
}

/**
 * V2 Data Transfer Object for indicator (camelCase)
 */
export interface IndicatorDtoV2 {
  label: string;
  totalCents: number;
  currency: string;
}

export interface IndicatorReportDtoV2 {
  indicators: IndicatorDtoV2[];
  refCurrency: string;
}

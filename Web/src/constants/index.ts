/**
 * Application-wide constants
 *
 * This file centralizes magic numbers and fixed values used throughout the application.
 * Values are stored in cents for monetary amounts to maintain precision.
 */

/**
 * Essential monthly living costs in cents
 * Used for calculating months of runway and financial independence metrics
 */
export const ESS_MONTHLY_COST_CENTS = 164000; // €1,640.00

/**
 * Discretionary spending monthly costs in cents
 * Additional spending beyond essential costs
 */
export const DST_MONTHLY_COST_CENTS = 125000; // €1,250.00

/**
 * Combined essential and discretionary monthly costs in cents
 */
export const TOTAL_MONTHLY_COST_CENTS = ESS_MONTHLY_COST_CENTS + DST_MONTHLY_COST_CENTS; // €2,890.00

/**
 * Expected annual investment return rate (as decimal)
 * Used for calculating investment income and financial independence metrics
 */
export const INVESTMENT_RETURN_RATE = 0.06; // 6% annual return

/**
 * Conversion factor from cents to currency units
 */
export const CENTS_TO_CURRENCY = 100.0;

/**
 * Grouped display order for the indicators table.
 * Each entry defines a group name and the ordered list of indicator labels in that group.
 * Indicators not listed in any group appear at the end under an implicit empty group.
 */
export type IndicatorConfig = { indicator: string; bold?: boolean; description?: string }

export const INDICATORS_INFO: { group: string; indicators: IndicatorConfig[] }[] = [
  {
    group: 'Net Worth',
    indicators: [
      { indicator: 'CASH', description: 'Cash' },
      { indicator: 'INVT', description: 'Investments' },
      { indicator: 'TONW', bold: true, description: 'Total Net Worth' }
    ]
  },
  {
    group: 'Cash Flow YTD',
    indicators: [
      { indicator: 'INPS', description: 'Income Passive' },
      { indicator: 'INAT', description: 'Income Active' },
      { indicator: 'INOO', description: 'Income One Off' },
      { indicator: 'EXPE', description: 'Expenses' },
      { indicator: 'CFOA', description: 'Cash Flow', bold: true }
    ]
  },
  {
    group: 'Past 12 Months',
    indicators: [
      { indicator: 'IP12', description: 'Income Passive' },
      { indicator: 'IA12', description: 'Income Active' },
      { indicator: 'OO12', description: 'Income One Off' },
      { indicator: 'EX12', description: 'Expenses' },
      { indicator: 'CF12', description: 'Cash Flow', bold: true }
    ]
  },
  {
    group: 'Budgets Alignment 12 Months',
    indicators: [
      { indicator: 'ES12', description: 'Expenses ESS' },
      { indicator: 'DS12', description: 'Expenses DST' },
    ]
  }
]

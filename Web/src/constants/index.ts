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

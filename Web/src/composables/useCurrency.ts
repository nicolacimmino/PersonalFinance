import { CENTS_TO_CURRENCY } from '@/constants';

/**
 * Currency Composable
 *
 * Provides currency conversion and formatting utilities.
 *
 * Usage:
 *   const { centsToAmount, amountToCents, formatCurrency } = useCurrency();
 */
export function useCurrency() {
  /**
   * Convert cents to currency amount
   * @param cents - Amount in cents
   * @returns Amount in currency units
   */
  function centsToAmount(cents: number): number {
    return cents / CENTS_TO_CURRENCY;
  }

  /**
   * Convert currency amount to cents
   * @param amount - Amount in currency units
   * @returns Amount in cents (rounded)
   */
  function amountToCents(amount: number): number {
    return Math.round(amount * CENTS_TO_CURRENCY);
  }

  /**
   * Format cents as currency string
   * @param cents - Amount in cents
   * @param currency - Currency symbol (default: '')
   * @param decimals - Number of decimal places (default: 2)
   * @returns Formatted currency string
   */
  function formatCurrency(
    cents: number,
    currency = '',
    decimals = 2
  ): string {
    const amount = centsToAmount(cents);
    const formatted = amount.toFixed(decimals);
    return currency ? `${formatted} ${currency}` : formatted;
  }

  /**
   * Format cents as currency with floor rounding
   * @param cents - Amount in cents
   * @param currency - Currency symbol (default: '')
   * @returns Formatted currency string (floored)
   */
  function formatCurrencyFloor(cents: number, currency = ''): string {
    const amount = Math.floor(centsToAmount(cents));
    return currency ? `${amount} ${currency}` : String(amount);
  }

  /**
   * Format currency for display with privacy support
   * @param cents - Amount in cents
   * @param currency - Currency symbol
   * @param isPrivate - Whether to hide the amount
   * @returns Formatted string or privacy placeholder
   */
  function formatWithPrivacy(
    cents: number,
    currency: string,
    isPrivate: boolean
  ): string {
    if (isPrivate) {
      return `--- ${currency}`;
    }
    return formatCurrency(cents, currency);
  }

  return {
    centsToAmount,
    amountToCents,
    formatCurrency,
    formatCurrencyFloor,
    formatWithPrivacy,
  };
}

function groupDigits(intStr: string): string {
  return intStr.replace(/\B(?=(\d{3})+(?!\d))/g, ' ');
}

/**
 * Format a cents value as a money string with 2 decimal places and space-grouped thousands.
 * e.g. formatMoney(1100050) → "11 000.50"
 */
export function formatMoney(cents: number): string {
  const negative = cents < 0;
  const abs = Math.abs(cents) / 100.0;
  const [intPart, decPart] = abs.toFixed(2).split('.');
  return `${negative ? '-' : ''}${groupDigits(intPart)}.${decPart}`;
}

/**
 * Format a plain integer with space-grouped thousands and no decimals.
 * e.g. formatCount(12345) → "12 345"
 */
export function formatCount(n: number): string {
  const negative = n < 0;
  const abs = String(Math.abs(Math.floor(n)));
  return `${negative ? '-' : ''}${groupDigits(abs)}`;
}

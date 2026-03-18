/**
 * Get current year with fallback
 */
function getCurrentYear(): number {
  try {
    const year = new Date().getFullYear();
    // Sanity check: year should be reasonable (between 2000 and 2100)
    if (year >= 2000 && year <= 2100) {
      return year;
    }
    // Fallback to 2026 if year is unreasonable
    return 2026;
  } catch (error) {
    // Fallback to 2026 if there's any error
    return 2026;
  }
}

/**
 * Parse date range parameters with defaults to current year
 * Matches the Rust API behavior
 */
export function parseDateRange(
  dateFromParam?: string | null,
  dateToParam?: string | null
): { dateFrom: string; dateTo: string } {
  const currentYear = getCurrentYear();

  // Handle undefined, null, empty string, or 'undefined' string
  const dateFrom = (dateFromParam && dateFromParam !== 'undefined' && dateFromParam.trim() !== '')
    ? dateFromParam
    : `${currentYear}-01-01`;

  const dateTo = (dateToParam && dateToParam !== 'undefined' && dateToParam.trim() !== '')
    ? dateToParam
    : `${currentYear}-12-31`;

  // Validate that we have valid date strings
  if (!dateFrom.match(/^\d{4}-\d{2}-\d{2}$/)) {
    throw new Error(`Invalid dateFrom format: ${dateFrom}`);
  }
  if (!dateTo.match(/^\d{4}-\d{2}-\d{2}$/)) {
    throw new Error(`Invalid dateTo format: ${dateTo}`);
  }

  return { dateFrom, dateTo };
}

/**
 * Format a Date object to PostgreSQL TIMESTAMP format
 * @param date Date object
 * @returns String in format 'YYYY-MM-DD HH:mm:ss'
 */
export function formatTimestamp(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  const hours = String(date.getHours()).padStart(2, '0');
  const minutes = String(date.getMinutes()).padStart(2, '0');
  const seconds = String(date.getSeconds()).padStart(2, '0');

  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
}

/**
 * Format a Date object to PostgreSQL DATE format
 * @param date Date object
 * @returns String in format 'YYYY-MM-DD'
 */
export function formatDate(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');

  return `${year}-${month}-${day}`;
}

/**
 * Validate date string format (YYYY-MM-DD)
 */
export function isValidDateFormat(dateStr: string): boolean {
  const regex = /^\d{4}-\d{2}-\d{2}$/;
  if (!regex.test(dateStr)) {
    return false;
  }

  const date = new Date(dateStr);
  return !isNaN(date.getTime());
}

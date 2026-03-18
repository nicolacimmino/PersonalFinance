import dotenv from 'dotenv';
import { logger } from './config/logger';
import { initDatabase, closePool } from './config/database';
import { TransformationService } from './service';
import { NewTransaction } from './model';

dotenv.config();

/**
 * Parse a date string (YYYY-MM-DD) to a Date object
 * Falls back to fallbackDate if parsing fails
 */
function parseDate(dateStr: string, fallbackDate?: Date): Date {
  if (!dateStr) {
    if (fallbackDate) return fallbackDate;
    throw new Error('No date provided and no fallback available');
  }

  const parsed = new Date(dateStr);
  if (isNaN(parsed.getTime())) {
    if (fallbackDate) return fallbackDate;
    throw new Error(`Invalid date: ${dateStr}`);
  }

  return parsed;
}

async function main(): Promise<void> {
  logger.info('Starting OBT (OpenBanking Transformer)');

  let exitCode = 0;

  try {
    await initDatabase();
    await transformTransactions();
  } catch (error) {
    logger.error('Fatal error in main', { error });
    exitCode = 1;
  } finally {
    await closePool();
    logger.info('Done');
    process.exit(exitCode);
  }
}

async function transformTransactions(): Promise<void> {
  const service = new TransformationService();

  const toTransform = await service.getUntransformedTransactions();

  logger.info(`Found ${toTransform.length} records`);

  for (const item of toTransform) {
    const obTransaction = item.ob_transaction;
    const account = item.account;

    // Determine transaction type based on amount
    const transactionType = obTransaction.transaction_amount_cents < 0 ? 'EXPENSE' : 'INCOME';

    // Most institutions report a booking and a value date. However, should some report only one
    // we default the other to same.
    const valueDate = parseDate(
      obTransaction.value_date,
      parseDate(obTransaction.booking_date)
    );

    const bookingDate = parseDate(obTransaction.booking_date, valueDate);

    const newTransaction: NewTransaction = {
      type_: transactionType,
      account_id: account.id,
      amount_cents: obTransaction.transaction_amount_cents,
      category: '',
      creditor_name: obTransaction.creditor_name,
      description: obTransaction.remittance_information_unstructured,
      booking_date: bookingDate,
      value_date: valueDate,
    };

    const newTransactionId = await service.insertTransaction(newTransaction);

    await service.updateObTransactionLink(obTransaction.id, newTransactionId);

    logger.info(`Converted ${obTransaction.id}`);
  }
}

// Run main
main();

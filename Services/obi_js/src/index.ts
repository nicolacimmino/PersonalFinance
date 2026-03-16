import dotenv from 'dotenv';
import { logger } from './config/logger';
import { initDatabase, closePool } from './config/database';
import { GoCardlessAccountsService, GoCardlessTransactionsService, GoCardlessTransaction } from './go_cardless';
import { OpenBankingAccountsService, OpenBankingTransactionsService, ObAccount, NewObTransaction } from './open_banking';

dotenv.config();

async function main(): Promise<void> {
  logger.info('Starting OBI (OpenBanking Import)');

  let exitCode = 0;

  try {
    await initDatabase();
    await syncAllAccountsTransactions();
  } catch (error) {
    logger.error('Fatal error in main', { error });
    exitCode = 1;
  } finally {
    await closePool();
    logger.info('Done');
    process.exit(exitCode);
  }
}

async function syncAllAccountsTransactions(): Promise<void> {
  const accountsService = new OpenBankingAccountsService();
  const obAccountsToSync = await accountsService.getAccountsToSync();

  logger.info(`Found ${obAccountsToSync.length} accounts to sync`);

  for (const obAccount of obAccountsToSync) {
    logger.info(`Syncing account ${obAccount.name} (${obAccount.id})`);

    try {
      await syncAccountTransactions(obAccount);
    } catch (error) {
      logger.error('Error in account sync', { error, accountId: obAccount.id });
    }
  }
}

async function syncAccountTransactions(obAccount: ObAccount): Promise<void> {
  const obAccountsService = new OpenBankingAccountsService();
  const obTransactionsService = new OpenBankingTransactionsService();
  const gcAccountsService = new GoCardlessAccountsService();
  const gcTransactionsService = new GoCardlessTransactionsService();

  logger.info('Syncing account status');

  // Get account info from GoCardless
  const accountInfo = await gcAccountsService.getAccount(obAccount.provider_account_id);

  // Update requisition status
  await obAccountsService.updateReqStatus(obAccount.id, accountInfo.status);

  // If the OB account is "READY" it means the requisition is done and still valid.
  // Anything else means we need to fix something.
  const accountStatus = accountInfo.status === 'READY' ? 'OK' : 'ERROR';

  await obAccountsService.updateAccountStatus(obAccount.account_id, accountStatus);

  if (accountStatus !== 'OK') {
    logger.error('Account link not ready. Skipping');
    return;
  }

  logger.info('Getting transactions');

  // Get transactions from GoCardless
  const transactions = await gcTransactionsService.getTransactions(obAccount.provider_account_id);
  const foundTransactions = transactions.length;

  logger.info(`Got ${foundTransactions} transactions from GoCardless`);

  let insertedTransactions = 0;

  for (const transaction of transactions) {
    // Check if transaction already exists
    const exists = await obTransactionsService.matchingTransactionExists(
      transaction.internalTransactionId,
      obAccount.id
    );

    if (exists) {
      continue;
    }

    logger.info(`Found new transaction ${transaction.transactionId}`);

    const newTransaction: NewObTransaction = {
      ob_account_id: obAccount.id,
      transaction_id: transaction.transactionId,
      booking_date: transaction.bookingDate,
      value_date: transaction.valueDate,
      booking_date_time: '', // TODO: drop from model, it's seldom and inconsistently populated
      transaction_amount_cents: transaction.transactionAmountCents,
      transaction_amount_currency: transaction.transactionAmountCurrency,
      creditor_name: transaction.creditorName,
      debtor_name: transaction.debtorName,
      debtor_account_iban: transaction.debtorAccountIban,
      remittance_information_unstructured: transaction.remittanceInformationUnstructured,
      balance_after_transaction_amount_cents: transaction.balanceAfterTransactionCents,
      balance_after_transaction_currency: transaction.transactionAmountCurrency,
      balance_after_transaction_type: transaction.balanceAfterTransactionType,
      internal_transaction_id: transaction.internalTransactionId,
    };

    await obTransactionsService.addTransaction(newTransaction);
    insertedTransactions++;
  }

  logger.info(`Added ${insertedTransactions} transactions, ${foundTransactions - insertedTransactions} already in DB`);

  logger.info('Updating last sync');

  await obAccountsService.updateLastSync(obAccount.id);
}

// Run main
main();

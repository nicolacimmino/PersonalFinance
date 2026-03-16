import { v4 as uuidv4 } from 'uuid';
import { query } from '../../config/database';
import { NewObTransaction } from '../model';

export class OpenBankingTransactionsService {
  /**
   * Check if a matching transaction already exists
   */
  async matchingTransactionExists(
    internalTransactionId: string,
    obAccountId: string
  ): Promise<boolean> {
    const result = await query<{ count: string }>(
      `SELECT COUNT(*) as count FROM raw.ob_transactions
       WHERE internal_transaction_id = $1 AND ob_account_id = $2`,
      [internalTransactionId, obAccountId]
    );

    return parseInt(result.rows[0].count, 10) > 0;
  }

  /**
   * Add a new transaction to the ob_transactions table
   */
  async addTransaction(transaction: NewObTransaction): Promise<void> {
    const id = uuidv4();

    await query(
      `INSERT INTO raw.ob_transactions (
        id,
        ob_account_id,
        transaction_id,
        booking_date,
        value_date,
        booking_date_time,
        transaction_amount_cents,
        transaction_amount_currency,
        creditor_name,
        debtor_name,
        debtor_account_iban,
        remittance_information_unstructured,
        balance_after_transaction_amount_cents,
        balance_after_transaction_currency,
        balance_after_transaction_type,
        internal_transaction_id
      ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)`,
      [
        id,
        transaction.ob_account_id,
        transaction.transaction_id,
        transaction.booking_date,
        transaction.value_date,
        transaction.booking_date_time,
        transaction.transaction_amount_cents,
        transaction.transaction_amount_currency,
        transaction.creditor_name,
        transaction.debtor_name,
        transaction.debtor_account_iban,
        transaction.remittance_information_unstructured,
        transaction.balance_after_transaction_amount_cents,
        transaction.balance_after_transaction_currency,
        transaction.balance_after_transaction_type,
        transaction.internal_transaction_id,
      ]
    );
  }
}

import { query } from '../config/database';
import { ObTransactionWithAccount, NewTransaction, Transaction } from '../model';

export class TransformationService {
  /**
   * Get all untransformed OB transactions with their related accounts
   */
  async getUntransformedTransactions(): Promise<ObTransactionWithAccount[]> {
    const result = await query<any>(
      `SELECT
        obt.id as obt_id,
        obt.ob_account_id,
        obt.transaction_id,
        obt.booking_date,
        obt.value_date,
        obt.booking_date_time,
        obt.transaction_amount_cents,
        obt.transaction_amount_currency,
        obt.creditor_name,
        obt.debtor_name,
        obt.debtor_account_iban,
        obt.remittance_information_unstructured,
        obt.balance_after_transaction_amount_cents,
        obt.balance_after_transaction_currency,
        obt.balance_after_transaction_type,
        obt.internal_transaction_id,
        oba.id as oba_id,
        oba.provider,
        oba.provider_account_id,
        oba.name as oba_name,
        oba.account_id,
        a.id as a_id,
        a.code,
        a.description,
        a.currency
      FROM raw.ob_transactions obt
      INNER JOIN raw.ob_accounts oba ON obt.ob_account_id = oba.id
      INNER JOIN raw.accounts a ON oba.account_id = a.id
      WHERE obt.transformed_transaction_id IS NULL`
    );

    return result.rows.map((row: any) => ({
      ob_transaction: {
        id: row.obt_id,
        ob_account_id: row.ob_account_id,
        transaction_id: row.transaction_id,
        booking_date: row.booking_date,
        value_date: row.value_date,
        booking_date_time: row.booking_date_time,
        transaction_amount_cents: row.transaction_amount_cents,
        transaction_amount_currency: row.transaction_amount_currency,
        creditor_name: row.creditor_name,
        debtor_name: row.debtor_name,
        debtor_account_iban: row.debtor_account_iban,
        remittance_information_unstructured: row.remittance_information_unstructured,
        balance_after_transaction_amount_cents: row.balance_after_transaction_amount_cents,
        balance_after_transaction_currency: row.balance_after_transaction_currency,
        balance_after_transaction_type: row.balance_after_transaction_type,
        internal_transaction_id: row.internal_transaction_id,
      },
      ob_account: {
        id: row.oba_id,
        provider: row.provider,
        provider_account_id: row.provider_account_id,
        name: row.oba_name,
        account_id: row.account_id,
      },
      account: {
        id: row.a_id,
        code: row.code,
        description: row.description,
        currency: row.currency,
      },
    }));
  }

  /**
   * Insert a new transaction and return its ID
   */
  async insertTransaction(transaction: NewTransaction): Promise<number> {
    const result = await query<{ id: number }>(
      `INSERT INTO raw.transactions (type, account_id, amount_cents, category, creditor_name, description, booking_date, value_date)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
       RETURNING id`,
      [
        transaction.type_,
        transaction.account_id,
        transaction.amount_cents,
        transaction.category,
        transaction.creditor_name,
        transaction.description,
        transaction.booking_date,
        transaction.value_date,
      ]
    );

    return result.rows[0].id;
  }

  /**
   * Update ob_transaction to link to the transformed transaction
   */
  async updateObTransactionLink(obTransactionId: string, transactionId: number): Promise<void> {
    await query(
      `UPDATE raw.ob_transactions SET transformed_transaction_id = $1 WHERE id = $2`,
      [transactionId, obTransactionId]
    );
  }
}

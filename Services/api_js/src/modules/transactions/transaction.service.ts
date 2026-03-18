import {query} from '../../config/database';
import {
    ApplicationTransaction,
    NewTransaction,
} from './transaction.model';
import {AppError} from '../../middleware/errorHandler';

/**
 * Get transactions with optional filters
 * @param category Optional category filter (LIKE pattern)
 * @param accountId Optional account ID filter
 * @param dateFrom Start date (YYYY-MM-DD)
 * @param dateTo End date (YYYY-MM-DD)
 * @returns Array of transactions
 */
export async function getTransactions(
    category: string | undefined,
    accountId: number | undefined,
    dateFrom: string,
    dateTo: string
): Promise<ApplicationTransaction[]> {
    const categoryPattern = `${category || ''}%`;

    const sql = `
        SELECT *
        FROM application.transactions
        WHERE category LIKE $1
          AND (account_id = $2 OR $2 IS NULL)
          AND (booking_date BETWEEN $3 AND $4)
        ORDER BY booking_date DESC
    `;

    const result = await query<ApplicationTransaction>(sql, [
        categoryPattern,
        accountId ?? null,
        `${dateFrom} 00:00:00`,
        `${dateTo} 00:00:00`,
    ]);

    return result.rows;
}

/**
 * Get a single transaction by ID
 * @param transactionId Transaction ID
 * @returns Transaction data
 * @throws AppError if transaction not found
 */
export async function getTransaction(
    transactionId: number
): Promise<ApplicationTransaction> {
    const sql = `
        SELECT *
        FROM application.transactions
        WHERE id = $1
    `;

    const result = await query<ApplicationTransaction>(sql, [transactionId]);

    if (result.rows.length === 0) {
        throw new AppError(404, `Transaction with id ${transactionId} not found`);
    }

    return result.rows[0];
}

/**
 * Update a transaction
 * @param transactionId Transaction ID
 * @param category Optional new category
 * @param type Optional new type
 * @param description Optional new description
 * @param accountTo Optional new account_to
 * @throws AppError if validation fails
 */
export async function updateTransaction(
    transactionId: number,
    category: string | null,
    type: string | null,
    description: string | null,
    accountTo: number | null
): Promise<void> {
    // Validation: Cannot set account_to if type is not TRANSFER
    if (type !== 'TRANSFER' && type !== undefined && accountTo !== null) {
        throw new AppError(
            400,
            'Cannot set account_to for non-TRANSFER transaction'
        );
    }

    const sql = `
        UPDATE raw.transactions
        SET category    = COALESCE($1, category),
            type        = COALESCE($2, type),
            description = COALESCE($3, description),
            account_to  = COALESCE($4, account_to)
        WHERE id = $5
    `;

    await query(sql, [
        category ?? null,
        type ?? null,
        description ?? null,
        accountTo ?? null,
        transactionId,
    ]);
}

/**
 * Create a new transaction
 * @param transaction New transaction data
 * @returns ID of the created transaction
 */
export async function createTransaction(
    transaction: NewTransaction
): Promise<number> {
    const sql = `
        INSERT INTO raw.transactions
        (type, account_id, amount_cents, category, creditor_name, description, booking_date, value_date, account_to)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id
    `;

    const result = await query<{ id: number }>(sql, [
        transaction.type_,
        transaction.account_id,
        transaction.amount_cents,
        transaction.category,
        transaction.creditor_name,
        transaction.description,
        transaction.booking_date,
        transaction.value_date,
        transaction.account_to,
    ]);

    return result.rows[0].id;
}

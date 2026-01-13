import { Request, Response } from 'express';
import * as service from '../transaction.service';
import {
  TransactionDtoV2,
  PatchTransactionDtoV2,
  CreateTransactionDtoV2,
} from './transaction.dto.v2';
import { ApplicationTransaction, NewTransaction } from '../transaction.model';
import { parseDateRange } from '../../../utils/dateHelpers';

/**
 * Transform database model to V2 DTO (camelCase)
 */
function toTransactionDtoV2(
  transaction: ApplicationTransaction
): TransactionDtoV2 {
  return {
    id: transaction.id,
    type: transaction.transaction_type,
    accountId: transaction.account_id,
    accountName: transaction.account_name,
    bookingDate: transaction.booking_date.toISOString(),
    category: transaction.category,
    creditorName: transaction.creditor_name,
    description: transaction.description,
    amountCents: transaction.amount_cents,
    currency: transaction.currency,
    amountCentsInRefCurrency: transaction.amount_cents_eur,
    refCurrency: 'EUR',
    accountTo: transaction.account_to,
    accountToName: transaction.account_to_name,
    receiptId: transaction.receipt_id,
  };
}

/**
 * GET /api/v2/transactions?category&account&dateFrom&dateTo
 * Get transactions with optional filters (camelCase query params)
 */
export async function getTransactions(
  req: Request,
  res: Response
): Promise<void> {
  // Accept both camelCase and snake_case for backward compatibility
  const { category, account, dateFrom, date_from, dateTo, date_to } = req.query;

  // Use camelCase first, fallback to snake_case
  const dateFromParam = dateFrom || date_from;
  const dateToParam = dateTo || date_to;

  // Parse date range with defaults
  const { dateFrom: parsedDateFrom, dateTo: parsedDateTo } = parseDateRange(
    dateFromParam ? String(dateFromParam) : undefined,
    dateToParam ? String(dateToParam) : undefined
  );

  // Parse account ID if provided
  const accountId = account ? parseInt(String(account), 10) : undefined;

  if (account && isNaN(accountId!)) {
    res.status(400).json({ error: 'Invalid account ID' });
    return;
  }

  const transactions = await service.getTransactions(
    category ? String(category) : undefined,
    accountId,
    parsedDateFrom,
    parsedDateTo
  );

  const dtos = transactions.map(toTransactionDtoV2);
  res.json(dtos);
}

/**
 * GET /api/v2/transactions/:id
 * Get a single transaction by ID
 */
export async function getTransaction(
  req: Request,
  res: Response
): Promise<void> {
  const id = parseInt(req.params.id, 10);

  if (isNaN(id)) {
    res.status(400).json({ error: 'Invalid transaction ID' });
    return;
  }

  const transaction = await service.getTransaction(id);
  const dto = toTransactionDtoV2(transaction);
  res.json(dto);
}

/**
 * POST /api/v2/transactions
 * Create a new transaction (accepts camelCase input)
 */
export async function createTransaction(
  req: Request,
  res: Response
): Promise<void> {
  const body: CreateTransactionDtoV2 = req.body;

  // Validate required fields
  if (!body.amountCents || !body.accountId) {
    res.status(400).json({ error: 'Missing required fields' });
    return;
  }

  // Map camelCase input to database model (snake_case)
  const newTransaction: NewTransaction = {
    type_: body.type || '',
    account_id: body.accountId,
    amount_cents: body.amountCents,
    category: body.category || '',
    creditor_name: body.creditorName || '',
    description: body.description || '',
    booking_date: body.bookingDate
      ? new Date(`${body.bookingDate} 00:00:00`)
      : new Date('1970-01-01 00:00:00'),
    value_date: body.valueDate
      ? new Date(`${body.valueDate} 00:00:00`)
      : new Date('1970-01-01 00:00:00'),
    account_to: body.accountTo ?? null,
  };

  // Create transaction and get the ID
  const id = await service.createTransaction(newTransaction);

  // Fetch the full enriched transaction
  const transaction = await service.getTransaction(id);
  const dto = toTransactionDtoV2(transaction);

  res.status(201).json(dto);
}

/**
 * PATCH /api/v2/transactions/:id
 * Update a transaction (accepts camelCase input)
 */
export async function patchTransaction(
  req: Request,
  res: Response
): Promise<void> {
  const id = parseInt(req.params.id, 10);

  if (isNaN(id)) {
    res.status(400).json({ error: 'Invalid transaction ID' });
    return;
  }

  const body: PatchTransactionDtoV2 = req.body;

  // Update transaction
  await service.updateTransaction(
    id,
    body.category,
    body.type,
    body.description,
    body.accountTo
  );

  // Fetch the updated transaction
  const transaction = await service.getTransaction(id);
  const dto = toTransactionDtoV2(transaction);

  res.json(dto);
}

import { Request, Response } from 'express';
import * as service from './transaction.service';
import {
  TransactionDto,
  PatchTransactionDto,
  CreateTransactionDto,
} from './transaction.dto';
import { ApplicationTransaction, NewTransaction } from './transaction.model';
import { parseDateRange } from '../../utils/dateHelpers';

/**
 * Transform database model to DTO
 */
function toTransactionDto(
  transaction: ApplicationTransaction
): TransactionDto {
  return {
    id: transaction.id,
    type: transaction.transaction_type,
    account_id: transaction.account_id,
    account_name: transaction.account_name,
    booking_date: transaction.booking_date.toISOString(),
    category: transaction.category,
    creditor_name: transaction.creditor_name,
    description: transaction.description,
    amount_cents: transaction.amount_cents,
    currency: transaction.currency,
    amount_cents_in_ref_currency: transaction.amount_cents_eur,
    ref_currency: 'EUR',
    account_to: transaction.account_to,
    account_to_name: transaction.account_to_name,
    receipt_id: transaction.receipt_id,
  };
}

/**
 * GET /api/transactions?category&account&date_from&date_to
 * Get transactions with optional filters
 */
export async function getTransactions(
  req: Request,
  res: Response
): Promise<void> {
  const { category, account, date_from, date_to } = req.query;

  // Parse date range with defaults (handle undefined properly)
  const { dateFrom, dateTo } = parseDateRange(
    date_from ? String(date_from) : undefined,
    date_to ? String(date_to) : undefined
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
    dateFrom,
    dateTo
  );

  const dtos = transactions.map(toTransactionDto);
  res.json(dtos);
}

/**
 * GET /api/transactions/:id
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
  const dto = toTransactionDto(transaction);
  res.json(dto);
}

/**
 * POST /api/transactions
 * Create a new transaction
 */
export async function createTransaction(
  req: Request,
  res: Response
): Promise<void> {
  const body: CreateTransactionDto = req.body;

  // Validate required fields
  if (!body.amount_cents || !body.account_id) {
    res.status(400).json({ error: 'Missing required fields' });
    return;
  }

  // Build NewTransaction with defaults
  const newTransaction: NewTransaction = {
    type_: body.type || '',
    account_id: body.account_id,
    amount_cents: body.amount_cents,
    category: body.category || '',
    creditor_name: body.creditor_name || '',
    description: body.description || '',
    booking_date: body.booking_date
      ? new Date(`${body.booking_date} 00:00:00`)
      : new Date('1970-01-01 00:00:00'),
    value_date: body.value_date
      ? new Date(`${body.value_date} 00:00:00`)
      : new Date('1970-01-01 00:00:00'),
    account_to: body.account_to ?? null,
  };

  // Create transaction and get the ID
  const id = await service.createTransaction(newTransaction);

  // Fetch the full enriched transaction
  const transaction = await service.getTransaction(id);
  const dto = toTransactionDto(transaction);

  res.status(201).json(dto);
}

/**
 * PATCH /api/transactions/:id
 * Update a transaction
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

  const body: PatchTransactionDto = req.body;

  // Update transaction
  await service.updateTransaction(
    id,
    body.category,
    body.type,
    body.description,
    body.account_to
  );

  // Fetch the updated transaction
  const transaction = await service.getTransaction(id);
  const dto = toTransactionDto(transaction);

  res.json(dto);
}

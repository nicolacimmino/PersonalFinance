import { Request, Response } from 'express';
import * as service from './account.service';
import { AccountDto } from './account.dto';
import { ApplicationAccount } from './account.model';

/**
 * Transform database model to DTO
 */
function toAccountDto(account: ApplicationAccount): AccountDto {
  return {
    id: account.id,
    code: account.code,
    description: account.description,
    currency: account.currency,
    iban: account.iban,
    status: account.status,
    asset_type: account.asset_type,
    pri_transactions_src: account.pri_transactions_src,
    balance_cents: account.balance_cents,
    balance_eur_cents: account.balance_eur_cents,
  };
}

/**
 * GET /api/accounts
 * Get all accounts
 */
export async function getAccounts(
  _req: Request,
  res: Response
): Promise<void> {
  const accounts = await service.getAccounts();
  const dtos = accounts.map(toAccountDto);
  res.json(dtos);
}

/**
 * GET /api/accounts/:id
 * Get a single account by ID
 */
export async function getAccount(req: Request, res: Response): Promise<void> {
  const id = parseInt(req.params.id, 10);

  if (isNaN(id)) {
    res.status(400).json({ error: 'Invalid account ID' });
    return;
  }

  const account = await service.getAccount(id);
  const dto = toAccountDto(account);
  res.json(dto);
}

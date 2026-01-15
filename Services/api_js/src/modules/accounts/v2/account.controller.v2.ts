import { Request, Response } from 'express';
import * as service from '../account.service';
import { AccountDtoV2 } from './account.dto.v2';
import { ApplicationAccount } from '../account.model';

/**
 * Transform database model to V2 DTO (camelCase)
 */
function toAccountDtoV2(account: ApplicationAccount): AccountDtoV2 {
  return {
    id: account.id,
    code: account.code,
    description: account.description,
    currency: account.currency,
    iban: account.iban,
    status: account.status,
    assetType: account.asset_type,
    balanceCents: account.balance_cents,
    balanceRefCurrencyCents: account.balance_eur_cents,
  };
}

/**
 * GET /api/v2/accounts
 * Get all accounts
 */
export async function getAccounts(
  _req: Request,
  res: Response
): Promise<void> {
  const accounts = await service.getAccounts();
  const dtos = accounts.map(toAccountDtoV2);
  res.json(dtos);
}

/**
 * GET /api/v2/accounts/:id
 * Get a single account by ID
 */
export async function getAccount(req: Request, res: Response): Promise<void> {
  const id = parseInt(req.params.id, 10);

  if (isNaN(id)) {
    res.status(400).json({ error: 'Invalid account ID' });
    return;
  }

  const account = await service.getAccount(id);
  const dto = toAccountDtoV2(account);
  res.json(dto);
}

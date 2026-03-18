import { query } from '../../config/database';
import { ObAccount } from '../model';

export class OpenBankingAccountsService {
  /**
   * Get all GoCardless accounts that need syncing
   * (last_sync older than 12 hours or NULL)
   */
  async getAccountsToSync(): Promise<ObAccount[]> {
    const result = await query<ObAccount>(
      `SELECT id, provider, provider_account_id, name, req_status, last_sync, account_id
       FROM raw.ob_accounts
       WHERE provider = 'GOCARDLESS'
         AND (last_sync < NOW() - INTERVAL '12 hours' OR last_sync IS NULL)`
    );

    return result.rows;
  }

  /**
   * Update the requisition status for an ob_account
   */
  async updateReqStatus(obAccountId: string, reqStatus: string): Promise<void> {
    await query(
      `UPDATE raw.ob_accounts SET req_status = $1 WHERE id = $2`,
      [reqStatus, obAccountId]
    );
  }

  /**
   * Update the last_sync timestamp for an ob_account
   */
  async updateLastSync(obAccountId: string): Promise<void> {
    await query(
      `UPDATE raw.ob_accounts SET last_sync = NOW() WHERE id = $1`,
      [obAccountId]
    );
  }

  /**
   * Update the status for an account
   */
  async updateAccountStatus(accountId: number, status: string): Promise<void> {
    await query(
      `UPDATE raw.accounts SET status = $1 WHERE id = $2`,
      [status, accountId]
    );
  }
}

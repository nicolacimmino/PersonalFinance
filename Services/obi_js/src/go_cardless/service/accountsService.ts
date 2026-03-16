import dotenv from 'dotenv';
import { GoCardlessApi } from '../goCardlessApi';
import { GoCardlessAccountInfo } from '../model';

dotenv.config();

export class GoCardlessAccountsService {
  private readonly secretId: string;
  private readonly secretKey: string;

  constructor() {
    const secretId = process.env.GOCARDLESS_SECRET_ID;
    const secretKey = process.env.GOCARDLESS_SECRET_KEY;

    if (!secretId) {
      throw new Error('GOCARDLESS_SECRET_ID must be set');
    }
    if (!secretKey) {
      throw new Error('GOCARDLESS_SECRET_KEY must be set');
    }

    this.secretId = secretId;
    this.secretKey = secretKey;
  }

  async getAccount(accountId: string): Promise<GoCardlessAccountInfo> {
    const api = new GoCardlessApi();
    await api.getToken(this.secretId, this.secretKey);

    const accountInfo = await api.getAccountInfo(accountId);

    return {
      status: accountInfo.status,
    };
  }
}

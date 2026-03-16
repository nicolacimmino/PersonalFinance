import dotenv from 'dotenv';
import { GoCardlessApi } from '../goCardlessApi';
import { GoCardlessTransaction, toGoCardlessTransaction } from '../model';

dotenv.config();

export class GoCardlessTransactionsService {
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

  async getTransactions(accountId: string): Promise<GoCardlessTransaction[]> {
    const api = new GoCardlessApi();
    await api.getToken(this.secretId, this.secretKey);

    const transactionDtos = await api.getTransactions(accountId);

    return transactionDtos.map(toGoCardlessTransaction);
  }
}

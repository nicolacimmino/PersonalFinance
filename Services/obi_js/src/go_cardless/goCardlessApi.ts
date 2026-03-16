import dotenv from 'dotenv';
import { logger } from '../config/logger';
import {
  CreateTokenRequest,
  CreateTokenResponse,
  GetTransactionsResponse,
  TransactionDto,
  AccountInfoResponse,
} from './dto';

dotenv.config();

export class GoCardlessApi {
  private accessToken: string = '';

  async getToken(secretId: string, secretKey: string): Promise<void> {
    const gocardlessHost = process.env.GOCARDLESS_HOST;
    if (!gocardlessHost) {
      throw new Error('GOCARDLESS_HOST environment variable is not set');
    }

    const request: CreateTokenRequest = {
      secret_id: secretId,
      secret_key: secretKey,
    };

    const response = await this.makePostRequest<CreateTokenRequest, CreateTokenResponse>(
      `${gocardlessHost}/api/v2/token/new/`,
      request
    );

    this.accessToken = response.access;
    logger.debug('GoCardless token obtained successfully');
  }

  async getTransactions(accountId: string): Promise<TransactionDto[]> {
    const gocardlessHost = process.env.GOCARDLESS_HOST;
    if (!gocardlessHost) {
      throw new Error('GOCARDLESS_HOST environment variable is not set');
    }

    const response = await this.makeGetRequest<GetTransactionsResponse>(
      `${gocardlessHost}/api/v2/accounts/${accountId}/transactions/`
    );

    return response.transactions.booked;
  }

  async getAccountInfo(accountId: string): Promise<AccountInfoResponse> {
    const gocardlessHost = process.env.GOCARDLESS_HOST;
    if (!gocardlessHost) {
      throw new Error('GOCARDLESS_HOST environment variable is not set');
    }

    const response = await this.makeGetRequest<AccountInfoResponse>(
      `${gocardlessHost}/api/v2/accounts/${accountId}/`
    );

    return response;
  }

  private async makePostRequest<TRequest, TResponse>(
    url: string,
    request: TRequest
  ): Promise<TResponse> {
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`POST request failed: ${response.status} ${errorText}`);
    }

    return response.json() as Promise<TResponse>;
  }

  private async makeGetRequest<TResponse>(url: string): Promise<TResponse> {
    const response = await fetch(url, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${this.accessToken}`,
      },
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`GET request failed: ${response.status} ${errorText}`);
    }

    return response.json() as Promise<TResponse>;
  }
}

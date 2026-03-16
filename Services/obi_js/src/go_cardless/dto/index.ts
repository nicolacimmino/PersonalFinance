// API Request/Response DTOs (match GoCardless API JSON structure with camelCase)

export interface CreateTokenRequest {
  secret_id: string;
  secret_key: string;
}

export interface CreateTokenResponse {
  access: string;
}

export interface AmountDto {
  amount: string;
  currency: string;
}

export interface AccountDto {
  iban: string;
}

export interface BalanceDto {
  balanceAmount: AmountDto;
  balanceType: string;
}

export interface TransactionDto {
  transactionId: string;
  bookingDate: string;
  valueDate?: string;
  bookingDateTime?: string;
  transactionAmount: AmountDto;
  creditorName?: string;
  debtorName?: string;
  debtorAccount?: AccountDto;
  remittanceInformationUnstructured?: string;
  remittanceInformationUnstructuredArray?: string[];
  balanceAfterTransaction?: BalanceDto;
  internalTransactionId: string;
}

export interface TransactionsDto {
  booked: TransactionDto[];
}

export interface GetTransactionsResponse {
  transactions: TransactionsDto;
}

export interface AccountInfoResponse {
  status: string;
}

import { TransactionDto } from '../dto';

// Domain model for a GoCardless transaction (converted from DTO)
export interface GoCardlessTransaction {
  transactionId: string;
  bookingDate: string;
  valueDate: string;
  transactionAmountCents: number;
  transactionAmountCurrency: string;
  creditorName: string;
  debtorName: string;
  debtorAccountIban: string;
  remittanceInformationUnstructured: string;
  balanceAfterTransactionCents: number;
  balanceAfterTransactionType: string;
  internalTransactionId: string;
}

// Domain model for GoCardless account info
export interface GoCardlessAccountInfo {
  status: string;
}

// Convert TransactionDto to GoCardlessTransaction
export function toGoCardlessTransaction(dto: TransactionDto): GoCardlessTransaction {
  const valueDate = dto.valueDate ?? dto.bookingDate;

  const transactionAmountCents = Math.round(parseFloat(dto.transactionAmount.amount) * 100);

  const debtorAccountIban = dto.debtorAccount?.iban ?? '';

  const remittanceInformationUnstructured =
    dto.remittanceInformationUnstructured ??
    (dto.remittanceInformationUnstructuredArray?.join(' ') ?? '');

  const balanceAfterTransaction = dto.balanceAfterTransaction ?? {
    balanceAmount: { amount: '0', currency: '' },
    balanceType: '',
  };

  const balanceAfterTransactionCents = Math.round(
    parseFloat(balanceAfterTransaction.balanceAmount.amount) * 100
  );

  const balanceAfterTransactionType = balanceAfterTransaction.balanceType;

  return {
    transactionId: dto.transactionId,
    bookingDate: dto.bookingDate,
    valueDate,
    transactionAmountCents,
    transactionAmountCurrency: dto.transactionAmount.currency,
    creditorName: dto.creditorName ?? '',
    debtorName: dto.debtorName ?? '',
    debtorAccountIban,
    remittanceInformationUnstructured,
    balanceAfterTransactionCents,
    balanceAfterTransactionType,
    internalTransactionId: dto.internalTransactionId,
  };
}

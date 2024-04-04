CREATE TABLE ob_transactions
(
    id                                     UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id                         TEXT       NOT NULL,
    booking_date                           TEXT       NOT NULL,
    value_date                             TEXT       NOT NULL,
    booking_date_time                      TEXT       NOT NULL,
    transaction_amount_cents               INT        NOT NULL,
    transaction_amount_currency            VARCHAR(3) NOT NULL,
    creditor_name                          TEXT       NOT NULL,
    debtor_name                            TEXT       NOT NULL,
    debtor_account_iban                    TEXT       NOT NULL,
    remittance_information_unstructured    TEXT       NOT NULL,
    balance_after_transaction_amount_cents INT        NOT NULL,
    balance_after_transaction_currency     VARCHAR(3) NOT NULL,
    balance_after_transaction_type         TEXT       NOT NULL,
    internal_transaction_id                TEXT       NOT NULL
)


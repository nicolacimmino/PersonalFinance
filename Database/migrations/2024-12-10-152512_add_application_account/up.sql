CREATE SCHEMA application;

CREATE VIEW application.accounts AS
SELECT accounts.id AS id,
       accounts.code AS code,
       accounts.description as description,
       accounts.currency AS currency,
       accounts.iban AS iban,
       accounts.status AS status,
       accounts.type AS asset_type,
       accounts.pri_transactions_src AS pri_transactions_src,
       SUM(amount_cents)::int4 AS balance_cents,
       (SUM(amount_cents)*valuta_conversion_rates.factor)::int4 AS balance_eur_cents
FROM accounts
JOIN transactions
    ON transactions.account_id=accounts.id
JOIN valuta_conversion_rates
    ON (
        valuta_conversion_rates.valuta_from = accounts.currency
        AND valuta_conversion_rates.valuta_to = 'EUR'
        )
GROUP BY accounts.id,valuta_conversion_rates.factor;

CREATE VIEW application.transactions AS
SELECT transactions.*,
       transactions.type AS movement_type,
       (transactions.amount_cents * valuta_conversion_rates_full.factor)::int AS amount_cents_eur,
       accounts.description AS account_name,
       accounts.currency,
       accounts.type as account_type,
       accounts_to.description AS account_to_name
FROM transactions
         LEFT JOIN accounts ON accounts.id = transactions.account_id
         LEFT JOIN accounts AS accounts_to ON accounts_to.id = transactions.account_to
         JOIN valuta_conversion_rates_full ON valuta_conversion_rates_full.valuta_from = accounts.currency
    AND valuta_conversion_rates_full.valuta_to = 'EUR';
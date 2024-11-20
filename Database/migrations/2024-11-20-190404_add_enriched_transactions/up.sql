CREATE MATERIALIZED VIEW transactions_enriched AS
SELECT transactions.*,
       (transactions.amount_cents * valuta_conversion_rates_full.factor)::int AS amount_cents_eur,
       accounts.currency,
       accounts.type as account_type
FROM transactions
         JOIN accounts ON accounts.id = transactions.account_id
         JOIN valuta_conversion_rates_full ON valuta_conversion_rates_full.valuta_from = accounts.currency
    AND valuta_conversion_rates_full.valuta_to = 'EUR';


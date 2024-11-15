
CREATE VIEW report_worth AS
SELECT
    concat('TOTAL_WORTH_EUR_',accounts.currency) as label,
    floor(SUM(transactions.amount_cents) * valuta_conversion_rates.factor) AS value
FROM accounts
         JOIN transactions ON
            transactions.account_id=accounts.id
         JOIN valuta_conversion_rates ON valuta_conversion_rates.valuta_from = accounts.currency
                AND valuta_conversion_rates.valuta_to = 'EUR'
GROUP BY valuta_conversion_rates.factor, accounts.currency
UNION
SELECT
    'TOTAL_WORTH_EUR' as label,
    floor(SUM(transactions.amount_cents * valuta_conversion_rates.factor)) AS value
FROM accounts
         JOIN transactions ON
            transactions.account_id=accounts.id
         JOIN valuta_conversion_rates ON valuta_conversion_rates.valuta_from = accounts.currency
                AND valuta_conversion_rates.valuta_to = 'EUR';


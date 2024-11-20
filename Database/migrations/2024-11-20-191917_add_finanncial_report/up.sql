CREATE VIEW financial_kpi AS
SELECT
    'CFY' AS label,
    sum(amount_cents_eur) AS amount_cents
FROM transactions_enriched
WHERE booking_date BETWEEN '2024-01-01' AND '2024-12-31'
  AND type <> 'INITIAL'
UNION
SELECT
    'TWO' AS label,
    sum(amount_cents_eur) AS amount_cents
FROM transactions_enriched
UNION
SELECT
    'CSH' AS label,
    sum(amount_cents_eur) AS amount_cents
FROM transactions_enriched
WHERE account_type='CASH'
UNION
SELECT
    'INV' AS label,
    sum(amount_cents_eur) AS amount_cents
FROM transactions_enriched
WHERE account_type='INV'
UNION
SELECT
    'BNK' AS label,
    sum(amount_cents_eur) AS amount_cents
FROM transactions_enriched
WHERE account_type like 'BANK_%';




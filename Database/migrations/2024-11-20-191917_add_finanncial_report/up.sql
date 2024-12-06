CREATE OR REPLACE FUNCTION get_kpis(
    date_from timestamp,
    date_to timestamp
)
    RETURNS TABLE
            (
                label        TEXT,
                amount_cents int4
            )
AS
$$
SELECT 'CFY.A'                                           AS label,
       sum(transactions_enriched.amount_cents_eur)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.booking_date >= date_from
  AND transactions_enriched.booking_date <= date_to
  AND transactions_enriched.type <> 'INITIAL' AND transactions_enriched.category not like 'PSV.%'
UNION
SELECT 'CFY.O'                                           AS label,
       sum(transactions_enriched.amount_cents_eur)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.booking_date >= date_from
  AND transactions_enriched.booking_date <= date_to
  AND transactions_enriched.type <> 'INITIAL'
UNION
SELECT 'TWO'                                             AS label,
       sum(transactions_enriched.amount_cents_eur)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.booking_date <= date_to
UNION
SELECT 'CSH'                                             AS label,
       sum(transactions_enriched.amount_cents_eur)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.account_type = 'CASH' OR transactions_enriched.account_type like 'BANK_%'
  AND transactions_enriched.booking_date <= date_to
UNION
SELECT 'INV'                                             AS label,
       sum(transactions_enriched.amount_cents_eur)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.account_type = 'INV'
  AND transactions_enriched.booking_date <= date_to
$$
    LANGUAGE sql;


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
SELECT 'CFAT'                                           AS label,
       coalesce(sum(transactions_enriched.amount_cents_eur),0)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.booking_date >= date_from
  AND transactions_enriched.booking_date <= date_to
  AND transactions_enriched.type <> 'INITIAL' AND transactions_enriched.category not like 'PSV.%'
UNION
SELECT 'INAT'                                           AS label,
       coalesce(sum(transactions_enriched.amount_cents_eur),0)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.booking_date >= date_from
  AND transactions_enriched.booking_date <= date_to
  AND transactions_enriched.type <> 'INITIAL' AND transactions_enriched.category like 'ACT.%'
UNION
SELECT 'INPS'                                           AS label,
       coalesce(sum(transactions_enriched.amount_cents_eur),0)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.booking_date >= date_from
  AND transactions_enriched.booking_date <= date_to
  AND transactions_enriched.type <> 'INITIAL' AND transactions_enriched.category like 'PSV.%'
UNION
SELECT 'CFOA'                                           AS label,
       coalesce(sum(transactions_enriched.amount_cents_eur),0)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.booking_date >= date_from
  AND transactions_enriched.booking_date <= date_to
  AND transactions_enriched.type <> 'INITIAL'
UNION
SELECT 'TONW'                                             AS label,
       coalesce(sum(transactions_enriched.amount_cents_eur),0)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.booking_date <= date_to
UNION
SELECT 'CASH'                                             AS label,
       coalesce(sum(transactions_enriched.amount_cents_eur),0)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.account_type = 'CASH' OR transactions_enriched.account_type like 'BANK_%'
  AND transactions_enriched.booking_date <= date_to
UNION
SELECT 'INVT'                                             AS label,
       coalesce(sum(transactions_enriched.amount_cents_eur),0)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.account_type = 'INV'
  AND transactions_enriched.booking_date <= date_to
UNION
SELECT CONCAT('C',transactions_enriched.currency) AS label,
       coalesce(sum(transactions_enriched.amount_cents_eur),0)::int4 AS amount_cents
FROM transactions_enriched
WHERE transactions_enriched.booking_date <= date_to
GROUP BY transactions_enriched.currency
$$
    LANGUAGE sql;




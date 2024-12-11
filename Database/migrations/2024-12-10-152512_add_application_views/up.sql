CREATE SCHEMA application;

CREATE VIEW application.categories AS
    SELECT * FROM raw.categories;

CREATE VIEW application.valuta_conversion_rates AS
    SELECT id,
           valuta_to AS valuta_from,
           valuta_from AS valuta_to,
           1/factor as factor
        FROM raw.valuta_conversion_rates
        UNION
            SELECT * FROM raw.valuta_conversion_rates;

CREATE VIEW application.accounts AS
SELECT raw.accounts.id AS id,
       raw.accounts.code AS code,
       raw.accounts.description as description,
       raw.accounts.currency AS currency,
       raw.accounts.iban AS iban,
       raw.accounts.status AS status,
       raw.accounts.type AS asset_type,
       raw.accounts.pri_transactions_src AS pri_transactions_src,
       SUM(amount_cents)::int4 AS balance_cents,
       (SUM(amount_cents)*application.valuta_conversion_rates.factor)::int4 AS balance_eur_cents
FROM raw.accounts
JOIN raw.transactions
    ON raw.transactions.account_id=accounts.id
JOIN application.valuta_conversion_rates
    ON (
        application.valuta_conversion_rates.valuta_from = accounts.currency
        AND application.valuta_conversion_rates.valuta_to = 'EUR'
        )
GROUP BY raw.accounts.id,application.valuta_conversion_rates.factor;

CREATE VIEW application.transactions AS
SELECT raw.transactions.*,
       raw.transactions.type AS movement_type,
       (raw.transactions.amount_cents * application.valuta_conversion_rates.factor)::int AS amount_cents_eur,
       application.accounts.description AS account_name,
       application.accounts.currency,
       application.accounts.asset_type as account_type,
       accounts_to.description AS account_to_name
FROM raw.transactions
         LEFT JOIN application.accounts ON application.accounts.id = raw.transactions.account_id
         LEFT JOIN application.accounts AS accounts_to ON accounts_to.id = raw.transactions.account_to
         JOIN application.valuta_conversion_rates ON application.valuta_conversion_rates.valuta_from = application.accounts.currency
    AND application.valuta_conversion_rates.valuta_to = 'EUR';

CREATE VIEW application.budgets AS (
SELECT A.*,
       SUM(A.spent_cents_eur * application.valuta_conversion_rates.factor)::int4 AS spent_cents
FROM (SELECT budgets.id,
             budgets.from_date,
             budgets.to_date,
             budgets.code,
             budgets.description,
             CURRENT_TIMESTAMP BETWEEN from_date and to_date                      AS active,
             budgets.currency,
             budgets.amount_cents,
             -SUM(application.transactions.amount_cents * application.valuta_conversion_rates.factor)::int4 AS spent_cents_eur,
             COUNT(*)::int4                                                             AS transactions
      FROM raw.budgets
               JOIN application.transactions ON
                  application.transactions.category like CONCAT(budgets.category_prefix, '%') AND
                  application.transactions.booking_date between budgets.from_date AND budgets.to_date
               JOIN application.accounts ON application.accounts.id = application.transactions.account_id
               JOIN application.valuta_conversion_rates ON application.valuta_conversion_rates.valuta_from = application.accounts.currency
          AND application.valuta_conversion_rates.valuta_to = 'EUR'
      GROUP BY budgets.id, budgets.code, budgets.description, budgets.currency,
               budgets.amount_cents) A
         JOIN application.valuta_conversion_rates ON application.valuta_conversion_rates.valuta_to = A.currency AND
                                              application.valuta_conversion_rates.valuta_from = 'EUR'
        GROUP BY A.id,A.from_date, A.to_date, A.code, A.description,A.active,A.currency,A.amount_cents,A.spent_cents_eur, A.transactions
);

CREATE VIEW application.alerts AS
(SELECT gen_random_uuid () as id,
       'WARN' as level,
       'BUDGETS' as item,
       'OVERSPENT' AS category,
       CONCAT('Budget overspent:', description) AS message,
       text(id) AS item_id
FROM application.budgets WHERE active=true AND spent_cents>application.budgets.amount_cents
UNION
SELECT gen_random_uuid () as id,
       'WARN' as level,
        'TRANSACTIONS' as item,
        'CATEGORY_MISSING' AS category,
        CONCAT('Missing category:', description) AS message,
       text(id) as item_id
       FROM application.transactions WHERE category='' AND type<>'TRANSFER' AND type<>'INITIAL' AND booking_date>='2024-01-01'
UNION
SELECT gen_random_uuid () as id,
       'ERROR' as level,
        'ACCOUNTS' as item,
        'ERROR' AS category,
       CONCAT('Error:', description) AS message,
       text(id)  AS item_id
       FROM application.accounts WHERE status<>'OK')
UNION
SELECT gen_random_uuid () as id,
       'WARN' as level,
        'ACCOUNTS' as item,
        'BALANCE' AS category,
       CONCAT('Negarive balance:', application.accounts.description) AS message,
       text(application.accounts.id)  AS item_id
        FROM application.accounts
            JOIN application.transactions on application.transactions.account_id=application.accounts.id
            GROUP BY application.accounts.description, application.accounts.id
            HAVING sum(amount_cents) < 0
UNION
SELECT gen_random_uuid () as id,
       'WARN' as level,
        'TRANSACTIONS' as item,
        'CATEGORY_INVALID' AS category,
        CONCAT('Invalid category:', category) AS message,
       text(id) as item_id
       FROM application.transactions WHERE category not in (select raw.categories.code from raw.categories)
                           AND category<>'' AND type<>'TRANSFER' AND type<>'INITIAL' AND booking_date>='2024-01-01'
ORDER BY item ASC, item_id ASC;

CREATE OR REPLACE FUNCTION application.get_indicators(
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
       coalesce(sum(application.transactions.amount_cents_eur),0)::int4 AS amount_cents
FROM application.transactions
WHERE application.transactions.booking_date >= date_from
  AND application.transactions.booking_date <= date_to
  AND application.transactions.type <> 'INITIAL' AND application.transactions.category not like 'PSV.%'
UNION
SELECT 'INAT'                                           AS label,
       coalesce(sum(application.transactions.amount_cents_eur),0)::int4 AS amount_cents
FROM application.transactions
WHERE application.transactions.booking_date >= date_from
  AND application.transactions.booking_date <= date_to
  AND application.transactions.type <> 'INITIAL' AND application.transactions.category like 'ACT.%'
UNION
SELECT 'INPS'                                           AS label,
       coalesce(sum(application.transactions.amount_cents_eur),0)::int4 AS amount_cents
FROM application.transactions
WHERE application.transactions.booking_date >= date_from
  AND application.transactions.booking_date <= date_to
  AND application.transactions.type <> 'INITIAL' AND application.transactions.category like 'PSV.%'
UNION
SELECT 'CFOA'                                           AS label,
       coalesce(sum(application.transactions.amount_cents_eur),0)::int4 AS amount_cents
FROM application.transactions
WHERE application.transactions.booking_date >= date_from
  AND application.transactions.booking_date <= date_to
  AND application.transactions.type <> 'INITIAL'
UNION
SELECT 'TONW'                                             AS label,
       coalesce(sum(application.transactions.amount_cents_eur),0)::int4 AS amount_cents
FROM application.transactions
WHERE application.transactions.booking_date <= date_to
UNION
SELECT 'CASH'                                             AS label,
       coalesce(sum(application.transactions.amount_cents_eur),0)::int4 AS amount_cents
FROM application.transactions
WHERE application.transactions.account_type = 'CASH' OR application.transactions.account_type like 'BANK_%'
  AND application.transactions.booking_date <= date_to
UNION
SELECT 'INVT'                                             AS label,
       coalesce(sum(application.transactions.amount_cents_eur),0)::int4 AS amount_cents
FROM application.transactions
WHERE application.transactions.account_type = 'INV'
  AND application.transactions.booking_date <= date_to
UNION
SELECT CONCAT('C',application.transactions.currency) AS label,
       coalesce(sum(application.transactions.amount_cents_eur),0)::int4 AS amount_cents
FROM application.transactions
WHERE application.transactions.booking_date <= date_to
GROUP BY application.transactions.currency
$$
    LANGUAGE sql;

CREATE TABLE budgets
(
    id              SERIAL PRIMARY KEY,
    code            VARCHAR(16) NOT NULL,
    description     TEXT        NOT NULL,
    category_prefix VARCHAR(32) NOT NULL,
    currency        VARCHAR(3)  NOT NULL,
    amount_cents    INT         NOT NULL,
    from_date       DATE   NOT NULL,
    to_date         DATE   NOT NULL,
    created_at      TIMESTAMP   NOT NULL DEFAULT current_timestamp
);


CREATE MATERIALIZED VIEW budgets_overview AS (
SELECT A.*,
       SUM(A.spent_cents_eur * valuta_conversion_rates_full.factor)::int4 AS spent_cents
FROM (SELECT budgets.id,
             budgets.from_date,
             budgets.to_date,
             budgets.code,
             budgets.description,
             CURRENT_TIMESTAMP BETWEEN from_date and to_date                      AS active,
             budgets.currency,
             budgets.amount_cents,
             -SUM(transactions.amount_cents * valuta_conversion_rates_full.factor)::int4 AS spent_cents_eur,
             COUNT(*)::int4                                                             AS transactions
      FROM budgets
               JOIN transactions ON
                  transactions.category like CONCAT(budgets.category_prefix, '%') AND
                  transactions.booking_date between budgets.from_date AND budgets.to_date
               JOIN accounts ON accounts.id = transactions.account_id
               JOIN valuta_conversion_rates_full ON valuta_conversion_rates_full.valuta_from = accounts.currency
          AND valuta_conversion_rates_full.valuta_to = 'EUR'
      GROUP BY budgets.id, budgets.code, budgets.description, budgets.currency,
               budgets.amount_cents) A
         JOIN valuta_conversion_rates_full ON valuta_conversion_rates_full.valuta_to = A.currency AND
                                              valuta_conversion_rates_full.valuta_from = 'EUR'
        GROUP BY A.id,A.from_date, A.to_date, A.code, A.description,A.active,A.currency,A.amount_cents,A.spent_cents_eur, A.transactions
);

CREATE MATERIALIZED VIEW budgets_details AS
(
SELECT budgets.id,
       budgets.code,
       transactions.category,
       budgets.description,
       budgets.currency,
       budgets.amount_cents,
       SUM(transactions.amount_cents * valuta_conversion_rates.factor) AS spent_cents_eur,
       COUNT(*)                                                        AS transactions
FROM budgets
         JOIN transactions ON
            transactions.category like CONCAT(budgets.category_prefix, '%') AND
            transactions.booking_date between budgets.from_date AND budgets.to_date
         JOIN accounts ON accounts.id = transactions.account_id
         JOIN valuta_conversion_rates ON valuta_conversion_rates.valuta_from = accounts.currency
    AND valuta_conversion_rates.valuta_to = 'EUR'
GROUP BY budgets.id, budgets.code, budgets.description, budgets.currency,
         budgets.amount_cents, transactions.category
);

CREATE VIEW valuta_conversion_rates_full AS
SELECT id,
        valuta_to AS valuta_from,
       valuta_from AS valuta_to,
       1/factor as factor
    FROM valuta_conversion_rates
    UNION
        SELECT * from valuta_conversion_rates;


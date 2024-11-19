CREATE VIEW alerts AS
(SELECT gen_random_uuid () as id,
       'WARN' as level,
       'BUDGETS' as item,
       'OVERSPENT' AS category,
       CONCAT('Budget overspent:', description) AS message,
       text(id) AS item_id
FROM budgets_overview WHERE active=true AND spent_cents>budgets_overview.amount_cents
UNION
SELECT gen_random_uuid () as id,
       'WARN' as level,
        'TRANSACTIONS' as item,
        'CATEGORY_MISSING' AS category,
        CONCAT('Missing category:', description) AS message,
       text(id) as item_id
       FROM transactions WHERE category='' AND type<>'TRANSFER' AND type<>'INITIAL' AND booking_date>='2024-01-01'
UNION
SELECT gen_random_uuid () as id,
       'ERROR' as level,
        'ACCOUNTS' as item,
        'ERROR' AS category,
       CONCAT('Error:', description) AS message,
       text(id)  AS item_id
       FROM accounts WHERE status<>'OK')
UNION
SELECT gen_random_uuid () as id,
       'WARN' as level,
        'ACCOUNTS' as item,
        'BALANCE' AS category,
       CONCAT('Negarive balance:', accounts.description) AS message,
       text(accounts.id)  AS item_id
        FROM accounts
            JOIN transactions on transactions.account_id=accounts.id
            GROUP BY accounts.description, accounts.id
            HAVING sum(amount_cents) < 0
UNION
SELECT gen_random_uuid () as id,
       'WARN' as level,
        'TRANSACTIONS' as item,
        'CATEGORY_INVALID' AS category,
        CONCAT('Invalid category:', category) AS message,
       text(id) as item_id
       FROM transactions WHERE category not in (select categories.code from categories)
                           AND category<>'' AND type<>'TRANSFER' AND type<>'INITIAL' AND booking_date>='2024-01-01'
ORDER BY item ASC, item_id ASC;
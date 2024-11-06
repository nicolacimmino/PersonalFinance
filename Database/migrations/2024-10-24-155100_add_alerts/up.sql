CREATE VIEW alerts AS
(SELECT gen_random_uuid () as id,
       'WARN' as level,
       'BUDGET' as item,
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
       FROM transactions WHERE category='' AND type!='TRANSFER'
UNION
SELECT gen_random_uuid () as id,
       'ERROR' as level,
        'ACCOUNTS' as item,
        'ERROR' AS category,
       CONCAT('Error:', description) AS message,
       text(id)  AS item_id
       FROM accounts WHERE status<>'OK')
ORDER BY item ASC, item_id ASC;

CREATE VIEW application.investments_contributions AS
    SELECT account_id,
           account_name,
           date_part('YEAR',booking_date) AS year,
           date_part('MONTH',booking_date) AS month,
           SUM(amount_cents_eur) AS total_cents_eur FROM application.transactions
    WHERE account_type='INV' AND transaction_type='TRANSFER'
    GROUP BY account_id,account_name,year,month
    ORDER BY year, month;

ALTER TABLE sp_transactions
    ADD COLUMN transformed_transaction_id INT REFERENCES transactions;

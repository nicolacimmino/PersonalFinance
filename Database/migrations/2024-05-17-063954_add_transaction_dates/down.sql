ALTER TABLE transactions
    ADD COLUMN date TIMESTAMP NOT NULL

ALTER TABLE transactions
DROP COLUMN value_date;

ALTER TABLE transactions
    DROP COLUMN booking_date;

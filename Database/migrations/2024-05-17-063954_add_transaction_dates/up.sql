
ALTER TABLE transactions
DROP COLUMN date;

ALTER TABLE transactions
    ADD COLUMN booking_date TIMESTAMP NOT NULL;

ALTER TABLE transactions
    ADD COLUMN value_date TIMESTAMP NOT NULL;


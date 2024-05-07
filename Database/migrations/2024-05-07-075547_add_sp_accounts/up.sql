create table sp_accounts
(
    id         UUID PRIMARY KEY       DEFAULT gen_random_uuid(),
    wallet     VARCHAR(128) NOT NULL,
    account_id INT REFERENCES accounts default 1
);

ALTER TABLE sp_transactions
    ADD COLUMN sp_account_id UUID REFERENCES sp_accounts NOT NULL ;

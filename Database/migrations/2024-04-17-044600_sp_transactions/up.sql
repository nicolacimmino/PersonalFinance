CREATE TABLE sp_transactions
(
    id       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date     VARCHAR NOT NULL,
    wallet   VARCHAR NOT NULL,
    type     VARCHAR NOT NULL,
    category VARCHAR NOT NULL,
    amount   VARCHAR NOT NULL,
    currency VARCHAR NOT NULL,
    note     VARCHAR NOT NULL,
    labels   VARCHAR NOT NULL,
    author   VARCHAR NOT NULL
);



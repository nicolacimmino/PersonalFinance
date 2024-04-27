CREATE TABLE accounts
(
    id          SERIAL PRIMARY KEY,
    code        VARCHAR(10) NOT NULL,
    description TEXT        NOT NULL,
    currency    VARCHAR(3)  NOT NULL
);

INSERT INTO accounts (id, code, description, currency)
VALUES (1, 'DUMMY', 'Dummy', 'DMB');

CREATE TABLE transactions
(
    id            SERIAL PRIMARY KEY,
    date          TIMESTAMP               NOT NULL,
    type          VARCHAR(16)             NOT NULL,
    account_id    INT references accounts NOT NULL,
    amount_cents  INT                     NOT NULL,
    category      TEXT                    NOT NULL,
    creditor_name TEXT                    NOT NULL,
    description   TEXT                    NOT NULL
);

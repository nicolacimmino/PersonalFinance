CREATE TABLE receipts
(
    id            SERIAL PRIMARY KEY,
    date          TIMESTAMP  NOT NULL,
    amount_cents  INT        NOT NULL,
    currency      VARCHAR(3) NOT NULL,
    ext_id        TEXT       NOT NULL,
    original_data TEXT       NOT NULL
);


CREATE TABLE receipts_line_items
(
    id               SERIAL PRIMARY KEY,
    receipt_id       INT REFERENCES receipts NOT NULL,
    quantity         NUMERIC(16, 3)          NOT NULL,
    unit_price_cents INT                     NOT NULL,
    amount_cents     INT                     NOT NULL,
    description      TEXT                    NOT NULL,
    raw_text         TEXT                    NOT NULL
);
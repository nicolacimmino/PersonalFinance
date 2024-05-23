CREATE TABLE valuta_conversion_rates
(
    id          SERIAL PRIMARY KEY,
    valuta_from VARCHAR(3)     NOT NULL,
    valuta_to   VARCHAR(3)     NOT NULL,
    factor      NUMERIC(16, 10) NOT NULL
);

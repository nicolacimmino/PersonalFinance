create table categories
(
    id         UUID PRIMARY KEY       DEFAULT gen_random_uuid(),
    code     VARCHAR(128) NOT NULL,
    color VARCHAR(6) NOT NULL
);

CREATE TABLE ob_accounts
(
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    provider            VARCHAR(128) NOT NULL,
    provider_account_id VARCHAR(128) NOT NULL
)

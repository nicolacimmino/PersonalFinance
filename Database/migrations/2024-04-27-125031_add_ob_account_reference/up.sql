
ALTER TABLE ob_accounts
 ADD COLUMN account_id INT NOT NULL DEFAULT 1 REFERENCES accounts;
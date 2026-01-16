ALTER TABLE users
    ADD COLUMN two_factor_secret TEXT,
ADD COLUMN two_factor_recovery_codes JSONB,
ADD COLUMN two_factor_confirmed_at TIMESTAMP;

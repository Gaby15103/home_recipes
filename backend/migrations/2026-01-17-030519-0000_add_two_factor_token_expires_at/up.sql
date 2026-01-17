-- Your SQL goes here
ALTER TABLE users
    ADD COLUMN two_factor_token_expires_at Timestamptz NULL;

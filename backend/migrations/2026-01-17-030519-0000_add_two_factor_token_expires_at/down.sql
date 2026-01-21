-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP
COLUMN two_factor_token_expires_at;-- This file should undo anything in `up.sql`

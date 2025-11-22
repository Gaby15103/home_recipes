-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users
(
    id              UUID PRIMARY KEY NOT NULL,
    email           VARCHAR(255) UNIQUE NOT NULL,
    username        VARCHAR(100) UNIQUE NOT NULL,

    -- Authentication
    password_hash   TEXT NOT NULL ,  -- NULL if using only social login

    -- Profile
    display_name    VARCHAR(150),
    avatar_url      TEXT,

    -- Preferences (JSON = flexibility)
    -- Preferences (JSON = flexibility)
    preferences     JSONB DEFAULT '{}',
    -- e.g. {"language": "en", "theme": "dark"}

    -- Status
    is_active       BOOLEAN DEFAULT TRUE,
    email_verified  BOOLEAN DEFAULT FALSE,

    -- Security timestamps
    last_login_at   TIMESTAMPTZ NULL,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

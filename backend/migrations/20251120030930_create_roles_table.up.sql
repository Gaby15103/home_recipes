-- Add up migration script here
CREATE TABLE IF NOT EXISTS roles
(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);
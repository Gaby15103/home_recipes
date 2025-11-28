-- Your SQL goes here
CREATE TABLE IF NOT EXISTS roles
(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

INSERT INTO roles (name, description)
VALUES
    ('USER', 'application user'),
    ('ADMIN', 'administrator'),
    ('MODERATOR', 'moderator'),
    ('SUPER_ADMIN', 'admin with all right');
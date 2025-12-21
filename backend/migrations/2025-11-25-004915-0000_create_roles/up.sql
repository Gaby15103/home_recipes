-- Your SQL goes here
CREATE TABLE IF NOT EXISTS roles
(
    id UUID PRIMARY KEY  DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

INSERT INTO roles (name, description)
VALUES
    ('USER', 'application user'),
    ('ADMIN', 'administrator'),
    ('MODERATOR', 'moderator'),
    ('SUPER_ADMIN', 'admin with all right');
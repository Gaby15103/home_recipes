-- Your SQL goes here
CREATE TABLE ingredients
(
    id           UUID PRIMARY KEY  DEFAULT uuid_generate_v4(),
    name         TEXT NOT NULL UNIQUE
);

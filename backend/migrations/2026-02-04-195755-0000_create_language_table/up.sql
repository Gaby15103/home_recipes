-- Your SQL goes here
CREATE TABLE languages
(
    code        VARCHAR(10) PRIMARY KEY,
    name        TEXT    NOT NULL,
    native_name TEXT    NOT NULL,
    is_active   BOOLEAN NOT NULL DEFAULT TRUE,
    is_default  BOOLEAN NOT NULL DEFAULT FALSE
);
INSERT INTO languages (code, name, native_name, is_default)
VALUES ('en', 'English', 'English', TRUE),
       ('fr', 'French', 'Français', FALSE),
       ('fr-CA', 'Canadian French', 'Français (Canada)', FALSE);

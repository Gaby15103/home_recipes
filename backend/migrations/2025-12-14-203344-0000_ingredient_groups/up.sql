-- Your SQL goes here
CREATE TABLE ingredient_groups
(
    id        UUID PRIMARY KEY  DEFAULT uuid_generate_v4(),
    recipe_id UUID    NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    title     TEXT    NOT NULL,
    position  INTEGER NOT NULL
);

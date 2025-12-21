-- Your SQL goes here
CREATE TABLE step_groups
(
    id        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    recipe_id UUID    NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    title     TEXT    NOT NULL,
    position  INTEGER NOT NULL,

    UNIQUE (recipe_id, position)
);

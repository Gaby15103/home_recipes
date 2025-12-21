-- Your SQL goes here
CREATE TABLE recipe_ingredients
(
    id                  UUID PRIMARY KEY  DEFAULT uuid_generate_v4(),
    ingredient_group_id UUID           NOT NULL
        REFERENCES ingredient_groups (id) ON DELETE CASCADE,
    ingredient_id       UUID           NOT NULL
        REFERENCES ingredients (id),
    quantity            NUMERIC(10, 2) NOT NULL,
    unit                TEXT           NOT NULL,
    note                TEXT,
    position            INTEGER        NOT NULL
);

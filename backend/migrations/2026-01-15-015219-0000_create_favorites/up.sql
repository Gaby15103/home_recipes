CREATE TABLE favorites
(
    user_id    UUID        NOT NULL,
    recipe_id  UUID        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (user_id, recipe_id),

    CONSTRAINT fk_favorites_user
        FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_favorites_recipe
        FOREIGN KEY (recipe_id)
            REFERENCES recipes (id)
            ON DELETE CASCADE
);

CREATE INDEX idx_favorites_recipe_id ON favorites (recipe_id);

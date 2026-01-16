CREATE TABLE recipe_ratings
(
    recipe_id  UUID        NOT NULL,
    user_id    UUID        NOT NULL,
    rating     INT         NOT NULL CHECK (rating BETWEEN 1 AND 5),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (recipe_id, user_id),

    CONSTRAINT fk_rating_recipe
        FOREIGN KEY (recipe_id)
            REFERENCES recipes (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_rating_user
        FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE CASCADE
);

CREATE INDEX idx_ratings_recipe ON recipe_ratings (recipe_id);

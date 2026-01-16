CREATE TABLE recipe_analytics
(
    id        UUID PRIMARY KEY  DEFAULT uuid_generate_v4(),
    recipe_id UUID        NOT NULL,
    user_id   UUID,
    viewed_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_analytics_recipe
        FOREIGN KEY (recipe_id)
            REFERENCES recipes (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_analytics_user
        FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE SET NULL
);

CREATE INDEX idx_analytics_recipe_id ON recipe_analytics (recipe_id);
CREATE INDEX idx_analytics_viewed_at ON recipe_analytics (viewed_at);

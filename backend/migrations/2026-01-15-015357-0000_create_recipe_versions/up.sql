CREATE TABLE recipe_versions
(
    id         UUID PRIMARY KEY  DEFAULT uuid_generate_v4(),
    recipe_id  UUID        NOT NULL,
    data       JSONB       NOT NULL,
    edited_by  UUID        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_versions_recipe
        FOREIGN KEY (recipe_id)
            REFERENCES recipes (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_versions_user
        FOREIGN KEY (edited_by)
            REFERENCES users (id)
            ON DELETE SET NULL
);

CREATE INDEX idx_versions_recipe_id ON recipe_versions (recipe_id);
CREATE INDEX idx_versions_created_at ON recipe_versions (created_at);

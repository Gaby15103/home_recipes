-- Your SQL goes here
CREATE TABLE recipe_comments
(
    id         UUID PRIMARY KEY  DEFAULT uuid_generate_v4(),
    recipe_id  UUID        NOT NULL,
    user_id    UUID        NOT NULL,
    parent_id  UUID,
    content    TEXT        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    edited_at  TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,

    CONSTRAINT fk_comment_recipe
        FOREIGN KEY (recipe_id)
            REFERENCES recipes (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_comment_user
        FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE SET NULL,

    CONSTRAINT fk_comment_parent
        FOREIGN KEY (parent_id)
            REFERENCES recipe_comments (id)
            ON DELETE CASCADE
);

CREATE INDEX idx_comments_recipe ON recipe_comments (recipe_id);
CREATE INDEX idx_comments_parent ON recipe_comments (parent_id);

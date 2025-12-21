-- Your SQL goes here
CREATE TABLE tags
(
    id   UUID PRIMARY KEY  DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE recipe_tags
(
    recipe_id UUID NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    tag_id    UUID NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
    PRIMARY KEY (recipe_id, tag_id)
);

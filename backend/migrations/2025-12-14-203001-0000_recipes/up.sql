-- Your SQL goes here
CREATE TABLE recipes
(
    id                UUID PRIMARY KEY  DEFAULT uuid_generate_v4(),
    title             TEXT        NOT NULL,
    description       TEXT,
    image_url         TEXT        NOT NULL,
    servings          INTEGER     NOT NULL,
    prep_time_minutes INTEGER     NOT NULL,
    cook_time_minutes INTEGER     NOT NULL,
    author            TEXT        NOT NULL,
    author_id         UUID            NULL REFERENCES users (id) ON DELETE CASCADE,
    is_private        BOOLEAN     NOT NULL DEFAULT true,
    created_at        TIMESTAMPTZ DEFAULT now(),
    updated_at        TIMESTAMPTZ DEFAULT now()
);

-- Your SQL goes here
CREATE TABLE steps
(
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    step_group_id    UUID REFERENCES step_groups (id) ON DELETE CASCADE NOT NULL,

    position         INTEGER NOT NULL,
    instruction      TEXT    NOT NULL,
    image_url        TEXT,
    duration_minutes INTEGER
);


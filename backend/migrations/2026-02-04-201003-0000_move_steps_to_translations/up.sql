CREATE TABLE step_group_translations
(
    id            UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    step_group_id UUID      NOT NULL,
    language_code TEXT      NOT NULL,

    title         TEXT      NOT NULL,

    created_at    TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_step_group
        FOREIGN KEY (step_group_id)
            REFERENCES step_groups (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_language
        FOREIGN KEY (language_code)
            REFERENCES languages (code)
            ON DELETE RESTRICT,

    CONSTRAINT unique_step_group_language
        UNIQUE (step_group_id, language_code)
);

CREATE INDEX idx_step_group_translations_group
    ON step_group_translations (step_group_id);

CREATE INDEX idx_step_group_translations_language
    ON step_group_translations (language_code);

CREATE TABLE step_translations
(
    id            UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    step_id       UUID      NOT NULL,
    language_code TEXT      NOT NULL,

    instruction   TEXT      NOT NULL,

    created_at    TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_step
        FOREIGN KEY (step_id)
            REFERENCES steps (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_language
        FOREIGN KEY (language_code)
            REFERENCES languages (code)
            ON DELETE RESTRICT,

    CONSTRAINT unique_step_language
        UNIQUE (step_id, language_code)
);

CREATE INDEX idx_step_translations_step
    ON step_translations (step_id);

CREATE INDEX idx_step_translations_language
    ON step_translations (language_code);


INSERT INTO step_group_translations (step_group_id,
                                     language_code,
                                     title,
                                     created_at,
                                     updated_at)
SELECT sg.id,
       r.original_language_code,
       sg.title,
       NOW(),
       NOW()
FROM step_groups sg
         JOIN recipes r ON r.id = sg.recipe_id;


INSERT INTO step_translations (step_id,
                               language_code,
                               instruction,
                               created_at,
                               updated_at)
SELECT s.id,
       r.original_language_code,
       s.instruction,
       NOW(),
       NOW()
FROM steps s
         JOIN step_groups sg ON sg.id = s.step_group_id
         JOIN recipes r ON r.id = sg.recipe_id;



ALTER TABLE step_groups
    DROP COLUMN title;

ALTER TABLE steps
    DROP COLUMN instruction;

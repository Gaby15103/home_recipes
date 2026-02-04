-- Your SQL goes here
CREATE TABLE ingredient_group_translations
(
    id                  UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    ingredient_group_id UUID      NOT NULL,
    language_code       TEXT      NOT NULL,

    title               TEXT      NOT NULL,

    created_at          TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_ingredient_group
        FOREIGN KEY (ingredient_group_id)
            REFERENCES ingredient_groups (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_language
        FOREIGN KEY (language_code)
            REFERENCES languages (code)
            ON DELETE RESTRICT,

    CONSTRAINT unique_ingredient_group_language
        UNIQUE (ingredient_group_id, language_code)
);

CREATE INDEX idx_ingredient_group_translations_group
    ON ingredient_group_translations (ingredient_group_id);

CREATE INDEX idx_ingredient_group_translations_language
    ON ingredient_group_translations (language_code);

CREATE TABLE ingredient_translations
(
    id            UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    ingredient_id UUID      NOT NULL,
    language_code TEXT      NOT NULL,

    name          TEXT      NOT NULL,

    created_at    TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_ingredient
        FOREIGN KEY (ingredient_id)
            REFERENCES ingredients (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_language
        FOREIGN KEY (language_code)
            REFERENCES languages (code)
            ON DELETE RESTRICT,

    CONSTRAINT unique_ingredient_language
        UNIQUE (ingredient_id, language_code)
);

CREATE INDEX idx_ingredient_translations_ingredient
    ON ingredient_translations (ingredient_id);

CREATE INDEX idx_ingredient_translations_language
    ON ingredient_translations (language_code);

CREATE TABLE recipe_ingredient_translations
(
    id                   UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    recipe_ingredient_id UUID      NOT NULL,
    language_code        TEXT      NOT NULL,

    note                 TEXT,

    created_at           TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_recipe_ingredient
        FOREIGN KEY (recipe_ingredient_id)
            REFERENCES recipe_ingredients (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_language
        FOREIGN KEY (language_code)
            REFERENCES languages (code)
            ON DELETE RESTRICT,

    CONSTRAINT unique_recipe_ingredient_language
        UNIQUE (recipe_ingredient_id, language_code)
);

CREATE INDEX idx_recipe_ingredient_translations_recipe_ingredient
    ON recipe_ingredient_translations (recipe_ingredient_id);

CREATE INDEX idx_recipe_ingredient_translations_language
    ON recipe_ingredient_translations (language_code);

INSERT INTO ingredient_group_translations (ingredient_group_id,
                                           language_code,
                                           title)
SELECT ig.id,
       r.original_language_code,
       ig.title
FROM ingredient_groups ig
         JOIN recipes r ON r.id = ig.recipe_id;

INSERT INTO ingredient_translations (ingredient_id,
                                     language_code,
                                     name)
SELECT i.id,
       (SELECT code FROM languages WHERE is_default = TRUE LIMIT 1),
       i.name
FROM ingredients i;


INSERT INTO recipe_ingredient_translations (recipe_ingredient_id,
                                            language_code,
                                            note)
SELECT ri.id,
       r.original_language_code,
       ri.note
FROM recipe_ingredients ri
         JOIN ingredient_groups ig ON ig.id = ri.ingredient_group_id
         JOIN recipes r ON r.id = ig.recipe_id
WHERE ri.note IS NOT NULL;

ALTER TABLE ingredient_groups
    DROP COLUMN title;
ALTER TABLE ingredients
    DROP COLUMN name;
ALTER TABLE recipe_ingredients
    DROP COLUMN note;

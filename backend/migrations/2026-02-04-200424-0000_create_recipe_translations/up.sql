-- Your SQL goes here
-- 1️⃣ Create translation table
CREATE TABLE recipe_translations
(
    id            UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    recipe_id     UUID      NOT NULL,
    language_code TEXT      NOT NULL,

    title         TEXT      NOT NULL,
    description   TEXT      NOT NULL,

    created_at    TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_recipe
        FOREIGN KEY (recipe_id)
            REFERENCES recipes (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_language
        FOREIGN KEY (language_code)
            REFERENCES languages (code)
            ON DELETE RESTRICT,

    CONSTRAINT unique_recipe_language
        UNIQUE (recipe_id, language_code)
);

CREATE INDEX idx_recipe_translations_recipe
    ON recipe_translations (recipe_id);

CREATE INDEX idx_recipe_translations_language
    ON recipe_translations (language_code);


-- 2️⃣ Ensure recipes has original_language_code
ALTER TABLE recipes
    ADD COLUMN IF NOT EXISTS original_language_code TEXT NOT NULL DEFAULT 'en';

ALTER TABLE recipes
    ADD CONSTRAINT fk_recipe_original_language
        FOREIGN KEY (original_language_code)
            REFERENCES languages (code);


-- 3️⃣ Move existing text into translations
INSERT INTO recipe_translations (recipe_id,
                                 language_code,
                                 title,
                                 description,
                                 created_at,
                                 updated_at)
SELECT id,
       original_language_code,
       title,
       description,
       created_at,
       created_at
FROM recipes;


-- 4️⃣ Remove text from recipes table
ALTER TABLE recipes
    DROP COLUMN title,
    DROP COLUMN description;

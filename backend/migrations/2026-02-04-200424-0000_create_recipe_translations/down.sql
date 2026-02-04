-- This file should undo anything in `up.sql`
-- 1️⃣ Restore columns
ALTER TABLE recipes
    ADD COLUMN title       TEXT,
    ADD COLUMN description TEXT;

-- 2️⃣ Restore values from translations
UPDATE recipes r
SET title       = rt.title,
    description = rt.description
FROM recipe_translations rt
WHERE rt.recipe_id = r.id
  AND rt.language_code = r.original_language_code;

-- 3️⃣ Drop translation table
DROP TABLE recipe_translations;

-- 4️⃣ Remove original_language_code
ALTER TABLE recipes
    DROP COLUMN original_language_code;

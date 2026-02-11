-- 1. Add columns back
ALTER TABLE ingredient_groups ADD COLUMN title TEXT;
ALTER TABLE ingredients ADD COLUMN name TEXT;
ALTER TABLE recipe_ingredients ADD COLUMN note TEXT;

-- 2. Corrected UPDATE (No 'ig' in FROM clause)
UPDATE ingredient_groups
SET title = igt.title
FROM ingredient_group_translations igt
         JOIN recipes r ON r.id = ingredient_groups.recipe_id
WHERE igt.ingredient_group_id = ingredient_groups.id
  AND igt.language_code = r.original_language_code;

-- 3. Corrected ingredients UPDATE
UPDATE ingredients
SET name = it.name
FROM ingredient_translations it
         JOIN languages l ON l.code = it.language_code
WHERE it.ingredient_id = ingredients.id
  AND l.is_default = TRUE;

-- 4. Corrected recipe_ingredients UPDATE
UPDATE recipe_ingredients
SET note = rit.note
FROM recipe_ingredient_translations rit
         JOIN ingredient_groups ig ON ig.id = recipe_ingredients.ingredient_group_id
         JOIN recipes r ON r.id = ig.recipe_id
WHERE rit.recipe_ingredient_id = recipe_ingredients.id
  AND rit.language_code = r.original_language_code;

-- 5. Drop the translation tables
DROP TABLE recipe_ingredient_translations;
DROP TABLE ingredient_translations;
DROP TABLE ingredient_group_translations;

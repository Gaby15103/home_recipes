-- This file should undo anything in `up.sql`
ALTER TABLE ingredient_groups ADD COLUMN title TEXT;
ALTER TABLE ingredients ADD COLUMN name TEXT;
ALTER TABLE recipe_ingredients ADD COLUMN note TEXT;


UPDATE ingredient_groups ig
SET title = igt.title
FROM ingredient_group_translations igt
         JOIN recipes r ON r.id = ig.recipe_id
WHERE igt.ingredient_group_id = ig.id
  AND igt.language_code = r.original_language_code;


UPDATE ingredients i
SET name = it.name
FROM ingredient_translations it
         JOIN languages l ON l.code = it.language_code
WHERE it.ingredient_id = i.id
  AND l.is_default = TRUE;


UPDATE recipe_ingredients ri
SET note = rit.note
FROM recipe_ingredient_translations rit
         JOIN ingredient_groups ig ON ig.id = ri.ingredient_group_id
         JOIN recipes r ON r.id = ig.recipe_id
WHERE rit.recipe_ingredient_id = ri.id
  AND rit.language_code = r.original_language_code;


DROP TABLE recipe_ingredient_translations;
DROP TABLE ingredient_translations;
DROP TABLE ingredient_group_translations;

-- This file should undo anything in `up.sql`
ALTER TABLE step_groups
    ADD COLUMN title TEXT;

ALTER TABLE steps
    ADD COLUMN instruction TEXT;


UPDATE step_groups sg
SET title = sgt.title
FROM step_group_translations sgt
         JOIN recipes r ON r.id = sg.recipe_id
WHERE
    sgt.step_group_id = sg.id
  AND sgt.language_code = r.original_language_code;


UPDATE steps s
SET instruction = st.instruction
FROM step_translations st
         JOIN step_groups sg ON sg.id = s.step_group_id
         JOIN recipes r ON r.id = sg.recipe_id
WHERE
    st.step_id = s.id
  AND st.language_code = r.original_language_code;


DROP TABLE step_translations;
DROP TABLE step_group_translations;

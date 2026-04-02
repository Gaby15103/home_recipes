-- ==========================================================
-- 1. SEEDING THE BRAIN (Lexicon & Basic Aliases)
-- ==========================================================

-- Standard Noise/Grammar Stopwords
INSERT OR IGNORE INTO lexicon (term_en, term_fr, category, priority, is_common)
VALUES ('with', 'avec', 'noise', 1, 1),
       ('and', 'et', 'noise', 1, 1),
       ('of', 'de', 'noise', 1, 1),
       ('for', 'pour', 'noise', 1, 1),
       ('in', 'dans', 'noise', 1, 1),
       ('the', 'le', 'noise', 1, 1),
       ('a', 'un', 'noise', 1, 1),
       ('or', 'ou', 'noise', 1, 1);

-- Units (Standard)
INSERT OR IGNORE INTO lexicon (term_en, term_fr, category, priority, is_common)
VALUES ('gram', 'gramme', 'unit', 1, 1),
       ('kilogram', 'kilogramme', 'unit', 1, 1),
       ('milliliter', 'millilitre', 'unit', 1, 1),
       ('liter', 'litre', 'unit', 1, 1),
       ('tablespoon', 'cuillère à soupe', 'unit', 1, 1),
       ('teaspoon', 'cuillère à café', 'unit', 1, 1),
       ('cup', 'tasse', 'unit', 1, 1),
       ('clove', 'gousse', 'unit', 2, 0),
       ('pinch', 'pincée', 'unit', 2, 0),
       ('can', 'boîte', 'unit', 2, 0),
       ('bunch', 'bouquet', 'unit', 2, 0),
       ('lb', 'livre', 'unit', 1, 1),
       ('oz', 'once', 'unit', 1, 1),
       ('dash', 'trait', 'unit', 2, 0),
       ('handful', 'poignée', 'unit', 2, 0),
       ('slice', 'tranche', 'unit', 2, 0),
       ('bag', 'sac', 'unit', 2, 0),
       ('bottle', 'bouteille', 'unit', 2, 0),
       ('head', 'tête', 'unit', 2, 0);

-- Units (Special Symbols)
INSERT OR IGNORE INTO lexicon (term_en, term_fr, category, priority, is_common)
VALUES ('celcius', '°c', 'unit', 1, 1),
       ('fahrenheit', '°f', 'unit', 1, 1),
       ('percent', '%', 'unit', 1, 1);

-- Actions (Cuts & Prep)
INSERT OR IGNORE INTO lexicon (term_en, term_fr, category, priority)
VALUES ('diced', 'en dés', 'action', 2),
       ('chopped', 'haché', 'action', 2),
       ('finely chopped', 'haché finement', 'action', 2),
       ('sliced', 'tranché', 'action', 2),
       ('minced', 'émincé', 'action', 2),
       ('julienned', 'en julienne', 'action', 2),
       ('shredded', 'râpé', 'action', 2),
       ('grated', 'râpé', 'action', 2),
       ('zested', 'zesté', 'action', 2),
       ('peeled', 'pelé', 'action', 2),
       ('pitted', 'dénoyauté', 'action', 2),
       ('mashed', 'écrasé', 'action', 2),
       ('pureed', 'en purée', 'action', 2),
       ('beaten', 'battu', 'action', 2),
       ('whisked', 'fouetté', 'action', 2),
       ('melted', 'fondu', 'action', 2),
       ('browned', 'doré', 'action', 2),
       ('toasted', 'grillé', 'action', 2),
       ('roasted', 'rôti', 'action', 2),
       ('boiled', 'bouilli', 'action', 2),
       ('steamed', 'cuit à la vapeur', 'action', 2),
       ('fried', 'frit', 'action', 2),
       ('sauteed', 'sauté', 'action', 2),
       ('baked', 'cuit au four', 'action', 2);

-- Descriptors
INSERT OR IGNORE INTO lexicon (term_en, term_fr, category, priority)
VALUES ('skinless', 'sans peau', 'descriptor', 1),
       ('boneless', 'désossé', 'descriptor', 1),
       ('deboned', 'désossée', 'descriptor', 1),
       ('detailed', 'détaillé', 'descriptor', 1),
       ('frozen', 'surgelé', 'descriptor', 2),
       ('fresh', 'frais', 'descriptor', 2),
       ('chilled', 'réfrigéré', 'descriptor', 2),
       ('dried', 'séché', 'descriptor', 2),
       ('fine julienne', 'fine julienne', 'descriptor', 1);

-- High-Priority Ingredients & Proteins
INSERT OR IGNORE INTO lexicon (term_en, term_fr, category, priority)
VALUES ('chicken', 'poulet', 'ingredient', 1),
       ('breast', 'poitrine', 'ingredient', 1),
       ('shrimp', 'crevettes', 'ingredient', 1),
       ('thigh', 'haut de cuisse', 'ingredient', 2),
       ('beef', 'boeuf', 'ingredient', 2),
       ('steak', 'bifteck', 'ingredient', 2),
       ('ground beef', 'boeuf haché', 'ingredient', 2),
       ('salt', 'sel', 'ingredient', 1),
       ('sugar', 'sucre', 'ingredient', 1),
       ('garlic', 'ail', 'ingredient', 1),
       ('coconut milk', 'lait de coco', 'ingredient', 1),
       ('onion', 'oignon', 'ingredient', 1),
       ('rice', 'riz', 'ingredient', 1);

-- Equipment
INSERT OR IGNORE INTO lexicon (term_en, term_fr, category, priority)
VALUES ('oven', 'four', 'equipment', 2),
       ('microwave', 'micro-ondes', 'equipment', 2),
       ('pan', 'poêle', 'equipment', 2),
       ('pot', 'marmite', 'equipment', 2),
       ('bowl', 'bol', 'equipment', 2),
       ('whisk', 'fouet', 'equipment', 2),
       ('blender', 'mélangeur', 'equipment', 2);

-- Headers (Classifier Support)
INSERT OR IGNORE INTO lexicon (term_en, term_fr, category, priority)
VALUES ('ingredients', 'ingrédients', 'text', 1),
       ('preparation', 'préparation', 'text', 1),
       ('directions', 'directions', 'text', 1),
       ('method', 'méthode', 'text', 1);

-- ==========================================================
-- 2. SEEDING THE MEMORY (Aliases & OCR Correction)
-- ==========================================================

-- Standard Abbreviations
INSERT OR IGNORE INTO aliases (raw_text, lexicon_id, confidence, is_verified)
VALUES ('g', (SELECT id FROM lexicon WHERE term_en = 'gram'), 1.0, 1),
       ('kg', (SELECT id FROM lexicon WHERE term_en = 'kilogram'), 1.0, 1),
       ('ml', (SELECT id FROM lexicon WHERE term_en = 'milliliter'), 1.0, 1),
       ('l', (SELECT id FROM lexicon WHERE term_en = 'liter'), 1.0, 1),
       ('tbsp', (SELECT id FROM lexicon WHERE term_en = 'tablespoon'), 1.0, 1),
       ('tsp', (SELECT id FROM lexicon WHERE term_en = 'teaspoon'), 1.0, 1),
       ('oz', (SELECT id FROM lexicon WHERE term_en = 'oz'), 1.0, 1);

-- French Unit Aliases
INSERT OR IGNORE INTO aliases (raw_text, lexicon_id, confidence, is_verified)
VALUES ('c. à soupe', (SELECT id FROM lexicon WHERE term_en = 'tablespoon'), 1.0, 1),
       ('c. à tab.', (SELECT id FROM lexicon WHERE term_en = 'tablespoon'), 1.0, 1),
       ('c àtab.', (SELECT id FROM lexicon WHERE term_en = 'tablespoon'), 1.0, 1),
       ('c. à café', (SELECT id FROM lexicon WHERE term_en = 'teaspoon'), 1.0, 1),
       ('c. à thé', (SELECT id FROM lexicon WHERE term_en = 'teaspoon'), 1.0, 1),
       ('c àthé', (SELECT id FROM lexicon WHERE term_en = 'teaspoon'), 1.0, 1);

-- OCR Corruption Specifics (The "Fucking" Fixes)
INSERT OR IGNORE INTO aliases (raw_text, lexicon_id, confidence, is_verified)
VALUES ('éaàthé', (SELECT id FROM lexicon WHERE term_en = 'teaspoon'), 0.9, 1),
       ('desel', (SELECT id FROM lexicon WHERE term_en = 'salt'), 0.9, 1);

-- ==========================================================
-- 3. SEEDING GRAMMAR RULES
-- ==========================================================
INSERT OR IGNORE INTO line_continuation_rules (pattern, rule_type, language_code)
VALUES ('et', 'SUFFIX', 'fr'),
       ('de', 'SUFFIX', 'fr'),
       ('ou', 'SUFFIX', 'fr'),
       ('d''', 'SUFFIX', 'fr'),
       ('l''', 'SUFFIX', 'fr'),
       ('avec', 'SUFFIX', 'fr'),
       ('sans', 'SUFFIX', 'fr'),
       ('en', 'SUFFIX', 'fr'),
       ('pour', 'SUFFIX', 'fr'),
       ('le', 'SUFFIX', 'fr'),
       ('la', 'SUFFIX', 'fr'),
       ('haché', 'PREFIX', 'fr'),
       ('émincé', 'PREFIX', 'fr'),
       ('coupé', 'PREFIX', 'fr'),
       ('en dés', 'PREFIX', 'fr'),
       ('tranché', 'PREFIX', 'fr'),
       ('and', 'SUFFIX', 'en'),
       ('with', 'SUFFIX', 'en'),
       ('chopped', 'PREFIX', 'en'),
       ('diced', 'PREFIX', 'en');

INSERT OR IGNORE INTO lexicon (term_en, term_fr, category, priority)
VALUES ('preparation', 'préPARATION', 'text', 1);
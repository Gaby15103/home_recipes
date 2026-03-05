-- 1. THE BRAIN (Standardized Terms)
CREATE TABLE lexicon
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    term_en    TEXT NOT NULL UNIQUE,
    term_fr    TEXT,
    term_zh    TEXT,               -- Added Chinese Support
    category   TEXT CHECK (category IN ('ingredient', 'unit', 'action', 'equipment', 'text')),
    priority   INTEGER  DEFAULT 3, -- 1=Common (Salt), 5=Rare (Saffron)
    is_common  BOOLEAN  DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 2. THE MEMORY (Learning from OCR/Users)
CREATE TABLE aliases
(
    raw_text    TEXT PRIMARY KEY,    -- The "messy" word found by OCR
    lexicon_id  INTEGER,
    confidence  REAL    DEFAULT 1.0, -- Matches based on previous user 'Acceptance'
    use_count   INTEGER DEFAULT 1,
    is_verified BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (lexicon_id) REFERENCES lexicon (id) ON DELETE CASCADE
);

-- 3. UNIT REGISTRY (For conversion & scaling)
CREATE TABLE units
(
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    lexicon_id        INTEGER, -- Links to Lexicon for naming
    abbreviation      TEXT,    -- 'g', 'ml', 'tbsp'
    is_metric         BOOLEAN DEFAULT TRUE,
    base_unit_id      INTEGER, -- For self-referencing (e.g., kg -> g)
    conversion_factor REAL,
    FOREIGN KEY (lexicon_id) REFERENCES lexicon (id)
);

-- 4. ANALYTICS & FEEDBACK (The "How Good Was It" Log)
CREATE TABLE parser_feedback_logs
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    raw_ocr_token      TEXT,    -- What Tesseract saw
    matched_lexicon_id INTEGER,
    match_strategy     TEXT,    -- 'exact_alias', 'fuzzy_levenshtein', 'ai_fallback'
    initial_confidence REAL,
    final_user_action  TEXT,    -- 'confirmed', 'edited', 'rejected'
    user_correction_id INTEGER, -- If they changed it, what did they pick?
    processing_time_ms INTEGER,
    created_at         DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (matched_lexicon_id) REFERENCES lexicon (id)
);

CREATE INDEX idx_lexicon_multi ON lexicon (term_en, term_fr, term_zh);
CREATE INDEX idx_alias_confidence ON aliases (confidence, use_count);
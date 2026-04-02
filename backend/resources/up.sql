-- 1. THE BRAIN (Standardized Terms)
CREATE TABLE IF NOT EXISTS lexicon
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    term_en    TEXT NOT NULL UNIQUE,
    term_fr    TEXT,
    term_zh    TEXT,
    category   TEXT CHECK (category IN ('ingredient', 'unit', 'action', 'equipment', 'text', 'descriptor', 'noise')),
    priority   INTEGER  DEFAULT 3,
    is_common  BOOLEAN  DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 2. THE MEMORY (Learning from OCR/Users)
CREATE TABLE IF NOT EXISTS aliases
(
    raw_text    TEXT PRIMARY KEY,
    lexicon_id  INTEGER,
    confidence  REAL    DEFAULT 1.0,
    use_count   INTEGER DEFAULT 1,
    is_verified BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (lexicon_id) REFERENCES lexicon (id) ON DELETE CASCADE
);

-- 3. UNIT REGISTRY
CREATE TABLE IF NOT EXISTS units
(
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    lexicon_id        INTEGER,
    abbreviation      TEXT,
    is_metric         BOOLEAN DEFAULT TRUE,
    base_unit_id      INTEGER,
    conversion_factor REAL,
    FOREIGN KEY (lexicon_id) REFERENCES lexicon (id)
);

-- 4. ANALYTICS & FEEDBACK
CREATE TABLE IF NOT EXISTS parser_feedback_logs
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    raw_ocr_token      TEXT,
    matched_lexicon_id INTEGER,
    match_strategy     TEXT,
    initial_confidence REAL,
    final_user_action  TEXT,
    user_correction_id INTEGER,
    processing_time_ms INTEGER,
    created_at         DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (matched_lexicon_id) REFERENCES lexicon (id)
);

-- 5. GRAMMAR RULES
CREATE TABLE IF NOT EXISTS line_continuation_rules
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    pattern       TEXT NOT NULL,
    rule_type     TEXT CHECK (rule_type IN ('PREFIX', 'SUFFIX')),
    language_code TEXT DEFAULT 'fr',
    UNIQUE(pattern, rule_type, language_code)
);

CREATE INDEX idx_lexicon_multi ON lexicon (term_en, term_fr, term_zh);
CREATE INDEX idx_alias_confidence ON aliases (confidence, use_count);

-- 6. FULL-TEXT SEARCH (FTS5)
CREATE VIRTUAL TABLE IF NOT EXISTS lexicon_fts USING fts5(
                                                             raw_text,
                                                             lexicon_id UNINDEXED,
                                                             tokenize='unicode61 remove_diacritics 1'
);

-- Initial population
INSERT OR IGNORE INTO lexicon_fts(raw_text, lexicon_id)
SELECT raw_text, lexicon_id FROM aliases;

INSERT OR IGNORE INTO lexicon_fts(raw_text, lexicon_id)
SELECT term_en, id FROM lexicon;

INSERT OR IGNORE INTO lexicon_fts(raw_text, lexicon_id)
SELECT term_fr, id FROM lexicon WHERE term_fr IS NOT NULL;

-- 7. AUTOMATION TRIGGERS (Safe Sync Pattern)

-- Alias Sync
CREATE TRIGGER IF NOT EXISTS aliases_ai AFTER INSERT ON aliases BEGIN
    INSERT INTO lexicon_fts(raw_text, lexicon_id) VALUES (new.raw_text, new.lexicon_id);
END;

CREATE TRIGGER IF NOT EXISTS aliases_ad AFTER DELETE ON aliases BEGIN
    DELETE FROM lexicon_fts WHERE raw_text = old.raw_text AND lexicon_id = old.lexicon_id;
END;

-- Lexicon Inserts
CREATE TRIGGER IF NOT EXISTS lexicon_ai AFTER INSERT ON lexicon BEGIN
    INSERT INTO lexicon_fts(raw_text, lexicon_id) VALUES (new.term_en, new.id);
    INSERT INTO lexicon_fts(raw_text, lexicon_id) SELECT new.term_fr, new.id WHERE new.term_fr IS NOT NULL;
END;

-- Lexicon Updates (Handling NULL to String transitions)
CREATE TRIGGER IF NOT EXISTS lexicon_au AFTER UPDATE ON lexicon BEGIN
    -- 1. Remove ANY existing FTS entries for this ID (English or French)
    DELETE FROM lexicon_fts WHERE lexicon_id = old.id;

    -- 2. Re-insert current values
    INSERT INTO lexicon_fts(raw_text, lexicon_id) VALUES (new.term_en, new.id);
    INSERT INTO lexicon_fts(raw_text, lexicon_id) SELECT new.term_fr, new.id WHERE new.term_fr IS NOT NULL;
END;

-- Lexicon Deletes
CREATE TRIGGER IF NOT EXISTS lexicon_ad AFTER DELETE ON lexicon BEGIN
    DELETE FROM lexicon_fts WHERE lexicon_id = old.id;
END;
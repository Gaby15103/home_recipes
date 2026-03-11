import asyncio
import aiosqlite
import pandas as pd
import aiohttp
import os

# Paths
FOOD_CSV = "./foodb_2020_04_07_csv/Food.csv"
DB_PATH = "dictionary.db"
TRANSLATOR_URL = "http://localhost:5000/translate"
CONCURRENCY_LIMIT = 20
UP_SQL_PATH = "./up.sql"
SEED_SQL_PATH = "./seed.sql"

async def run_sql_file(db, path):
    """Helper to execute an entire .sql file."""
    if os.path.exists(path):
        print(f"📖 Executing {path}...")
        with open(path, 'r', encoding='utf-8') as f:
            script = f.read()
        try:
            await db.executescript(script)
            await db.commit()
            print(f"✅ {path} applied.")
        except Exception as e:
            print(f"❌ Error in {path}: {e}")
            raise e
    else:
        print(f"⚠️ Warning: {path} not found.")

async def translate_item(session, en_name, semaphore):
    async with semaphore:
        try:
            payload = {"q": en_name, "source": "en", "target": "fr", "format": "text"}
            async with session.post(TRANSLATOR_URL, json=payload, timeout=5) as resp:
                if resp.status == 200:
                    data = await resp.json()
                    translated = data.get("translatedText", "").lower().strip()
                    return translated if translated else None
        except:
            return None
    return None

async def upsert_lexicon(db, en, fr, cat, prio):
    en = en.lower().strip()
    # Treat empty strings as None for the FTS triggers
    fr = fr.lower().strip() if (fr and fr.strip()) else None

    try:
        # Insert or Update main Lexicon entry
        await db.execute("""
                         INSERT INTO lexicon (term_en, term_fr, category, priority)
                         VALUES (?, ?, ?, ?)
                         ON CONFLICT(term_en) DO UPDATE SET
                                                            term_fr = COALESCE(lexicon.term_fr, excluded.term_fr),
                                                            category = excluded.category,
                                                            updated_at = CURRENT_TIMESTAMP
                         """, (en, fr, cat, prio))

        # Retrieve ID for alias linking
        async with db.execute("SELECT id FROM lexicon WHERE term_en = ?", (en,)) as cursor:
            row = await cursor.fetchone()
            if row:
                lex_id = row[0]
                await db.execute("INSERT OR IGNORE INTO aliases (raw_text, lexicon_id) VALUES (?, ?)", (en, lex_id))
                if fr:
                    await db.execute("INSERT OR IGNORE INTO aliases (raw_text, lexicon_id) VALUES (?, ?)", (fr, lex_id))
    except sqlite3.OperationalError as e:
        print(f"⚠️ SQL Error on {en}: {e}")
        raise e

async def main():
    print(f"🔍 Starting Migration on: {DB_PATH}")

    async with aiosqlite.connect(DB_PATH) as db:
        # Optimization for massive batch inserts
        await db.execute("PRAGMA journal_mode=WAL;")
        await db.execute("PRAGMA synchronous=NORMAL;")

        # 1. Apply Schema and Manual Seeds
        await run_sql_file(db, UP_SQL_PATH)
        await run_sql_file(db, SEED_SQL_PATH)

        # 2. Process FooDB
        if os.path.exists(FOOD_CSV):
            print("⏳ Processing FooDB items...")
            df = pd.read_csv(FOOD_CSV, engine='python', on_bad_lines='skip')
            items = df[df['name'].notna()]['name'].str.strip().tolist()

            semaphore = asyncio.Semaphore(CONCURRENCY_LIMIT)
            async with aiohttp.ClientSession() as session:
                for i in range(0, len(items), 50):
                    batch = items[i:i+50]
                    tasks = [translate_item(session, name, semaphore) for name in batch]
                    translations = await asyncio.gather(*tasks)

                    try:
                        for en_raw, fr_name in zip(batch, translations):
                            await upsert_lexicon(db, en_raw, fr_name, 'ingredient', 3)
                        await db.commit() # Commit batch
                    except Exception as e:
                        print(f"❌ Batch failed at index {i}: {e}")
                        # If a batch fails, we don't want to lose everything
                        await db.rollback()
                        continue

                    if i % 500 == 0:
                        print(f"🚀 Progress: {i}/{len(items)}")

        print("🏁 Migration complete. Manual seeds preserved.")

if __name__ == "__main__":
    asyncio.run(main())
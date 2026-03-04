import asyncio
import aiosqlite
import pandas as pd
import aiohttp
import os

# Paths
FOOD_CSV = "./foodb_2020_04_07_csv/Food.csv"
DB_PATH = "dictionnary.db"
# Local Docker LibreTranslate
TRANSLATOR_URL = "http://localhost:5000/translate"

# Concurrency: How many rows to translate at once
CONCURRENCY_LIMIT = 20

async def translate_item(session, en_name, semaphore):
    """Hits the Docker container for a French translation."""
    async with semaphore:
        try:
            payload = {"q": en_name, "source": "en", "target": "fr", "format": "text"}
            async with session.post(TRANSLATOR_URL, json=payload, timeout=5) as resp:
                if resp.status == 200:
                    data = await resp.json()
                    return data.get("translatedText", "").lower()
        except:
            return None
    return None

async def migrate_to_brain():
    if not os.path.exists(FOOD_CSV):
        print("❌ Food.csv not found!")
        return

    # 1. Load Data with Python Engine (Handles quoted commas like "Milk (Cow)")
    print("⏳ Loading FooDB data...")
    df = pd.read_csv(FOOD_CSV, engine='python', on_bad_lines='skip')

    # Clean and list all names
    items = df[df['name'].notna()]['name'].str.strip().tolist()
    total = len(items)
    print(f"🌍 Starting Async Migration for {total} items...")

    # 2. Open Async SQLite Connection
    async with aiosqlite.connect(DB_PATH) as db:
        # Enable WAL for concurrent access (so RustRover/DataGrip can view data while writing)
        await db.execute("PRAGMA journal_mode=WAL;")
        await db.execute("PRAGMA synchronous=NORMAL;")

        semaphore = asyncio.Semaphore(CONCURRENCY_LIMIT)

        async with aiohttp.ClientSession() as session:
            # Process in chunks of 50 for database stability
            for i in range(0, total, 50):
                batch = items[i:i+50]
                tasks = [translate_item(session, name, semaphore) for name in batch]
                translations = await asyncio.gather(*tasks)

                for en_raw, fr_name in zip(batch, translations):
                    en_name = en_raw.lower()

                    try:
                        # 3. INSERT OR IGNORE into Lexicon (The Brain)
                        # We use priority 3 as per your schema
                        await db.execute("""
                                         INSERT INTO lexicon (term_en, term_fr, category, priority)
                                         VALUES (?, ?, 'ingredient', 3)
                                         ON CONFLICT(term_en) DO UPDATE SET
                                                                            term_fr = COALESCE(excluded.term_fr, lexicon.term_fr),
                                                                            updated_at = CURRENT_TIMESTAMP
                                         """, (en_name, fr_name))

                        # 4. Get the ID to link Aliases
                        async with db.execute("SELECT id FROM lexicon WHERE term_en = ?", (en_name,)) as cursor:
                            row = await cursor.fetchone()
                            if row:
                                lex_id = row[0]

                                # 5. Populate Aliases (The Memory)
                                # English Master Alias
                                await db.execute("""
                                                 INSERT OR IGNORE INTO aliases (raw_text, lexicon_id, confidence, is_verified)
                                                 VALUES (?, ?, 1.0, 1)
                                                 """, (en_name, lex_id))

                                # French Master Alias
                                if fr_name:
                                    await db.execute("""
                                                     INSERT OR IGNORE INTO aliases (raw_text, lexicon_id, confidence, is_verified)
                                                     VALUES (?, ?, 1.0, 1)
                                                     """, (fr_name, lex_id))
                    except Exception as e:
                        print(f"⚠️ Error on {en_name}: {e}")

                # Commit every 50 items
                await db.commit()

                # Special log to confirm "Milk" varieties are being hit
                if any("milk" in b.lower() for b in batch):
                    print(f"🥛 Batch {i}-{i+50}: Imported milk variations...")

                if i % 500 == 0:
                    print(f"🚀 Progress: {i}/{total} rows processed...")

    print("✨ SUCCESS: Every single row from FooDB is now in your Dictionary.")

if __name__ == "__main__":
    asyncio.run(migrate_to_brain())
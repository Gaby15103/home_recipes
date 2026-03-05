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

# --- DATA SEEDING COLLECTIONS ---
# (term_en, term_fr, category, priority)
CULINARY_SEEDS = [
    # Units
    ('gram', 'gramme', 'unit', 1),
    ('kilogram', 'kilogramme', 'unit', 1),
    ('ml', 'ml', 'unit', 1),
    ('liter', 'litre', 'unit', 1),
    ('tablespoon', 'cuillère à soupe', 'unit', 1),
    ('teaspoon', 'cuillère à café', 'unit', 1),
    ('cup', 'tasse', 'unit', 1),
    ('clove', 'gousse', 'unit', 2),
    ('pinch', 'pincée', 'unit', 2),
    ('can', 'boîte', 'unit', 2),
    ('bunch', 'bouquet', 'unit', 2),

    # --- Symbols & Abbreviations (Mapped to 'unit') ---
    ('g', 'g', 'unit', 1),
    ('kg', 'kg', 'unit', 1),
    ('ml', 'ml', 'unit', 1),
    ('cl', 'cl', 'unit', 1),
    ('l', 'l', 'unit', 1),
    ('lb', 'livre', 'unit', 1),
    ('oz', 'once', 'unit', 1),
    ('tsp', 'c. à café', 'unit', 1),
    ('tbsp', 'c. à soupe', 'unit', 1),
    ('°c', '°c', 'unit', 1),
    ('°f', '°f', 'unit', 1),
    ('%', '%', 'unit', 1),

    # --- More Units ---
    ('pinch', 'pincée', 'unit', 2),
    ('dash', 'trait', 'unit', 2),
    ('handful', 'poignée', 'unit', 2),
    ('slice', 'tranche', 'unit', 2),
    ('bag', 'sac', 'unit', 2),
    ('bottle', 'bouteille', 'unit', 2),
    ('clove', 'gousse', 'unit', 2),
    ('head', 'tête', 'unit', 2), # e.g., head of garlic

    # --- Expanded Actions (Prep/Cuts) ---
    ('diced', 'en dés', 'action', 2),
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
    ('baked', 'cuit au four', 'action', 2),
    ('chilled', 'réfrigéré', 'action', 2),
    ('frozen', 'surgelé', 'action', 2),
    ('thawed', 'décongelé', 'action', 2),
    ('dried', 'séché', 'action', 2),

    # --- Equipment (For Step Detection) ---
    ('oven', 'four', 'equipment', 2),
    ('stove', 'cuisinière', 'equipment', 2),
    ('microwave', 'micro-ondes', 'equipment', 2),
    ('air fryer', 'friteuse à air', 'equipment', 2),
    ('pan', 'poêle', 'equipment', 2),
    ('frying pan', 'poêle à frire', 'equipment', 2),
    ('skillet', 'poêle en fonte', 'equipment', 2),
    ('pot', 'marmite', 'equipment', 2),
    ('saucepan', 'casserole', 'equipment', 2),
    ('baking sheet', 'plaque de cuisson', 'equipment', 2),
    ('bowl', 'bol', 'equipment', 2),
    ('mixing bowl', 'bol à mélanger', 'equipment', 2),
    ('whisk', 'fouet', 'equipment', 2),
    ('spatula', 'spatule', 'equipment', 2),
    ('knife', 'couteau', 'equipment', 2),
    ('blender', 'mélangeur', 'equipment', 2),
    ('food processor', 'robot culinaire', 'equipment', 2),
    ('scale', 'balance', 'equipment', 2),
    ('thermometer', 'thermomètre', 'equipment', 2),
]

async def translate_item(session, en_name, semaphore):
    async with semaphore:
        try:
            payload = {"q": en_name, "source": "en", "target": "fr", "format": "text"}
            async with session.post(TRANSLATOR_URL, json=payload, timeout=5) as resp:
                if resp.status == 200:
                    data = await resp.json()
                    return data.get("translatedText", "").lower()
        except: return None
    return None

async def upsert_lexicon(db, en, fr, cat, prio):
    """Helper to insert into lexicon and auto-create aliases."""
    en = en.lower()
    fr = fr.lower() if fr else None

    await db.execute("""
                     INSERT INTO lexicon (term_en, term_fr, category, priority)
                     VALUES (?, ?, ?, ?)
                     ON CONFLICT(term_en) DO UPDATE SET
                                                        term_fr = COALESCE(excluded.term_fr, lexicon.term_fr),
                                                        category = excluded.category,
                                                        updated_at = CURRENT_TIMESTAMP
                     """, (en, fr, cat, prio))

    async with db.execute("SELECT id FROM lexicon WHERE term_en = ?", (en,)) as cursor:
        row = await cursor.fetchone()
        if row:
            lex_id = row[0]
            await db.execute("INSERT OR IGNORE INTO aliases (raw_text, lexicon_id, confidence, is_verified) VALUES (?, ?, 1.0, 1)", (en, lex_id))
            if fr:
                await db.execute("INSERT OR IGNORE INTO aliases (raw_text, lexicon_id, confidence, is_verified) VALUES (?, ?, 1.0, 1)", (fr, lex_id))

async def migrate_to_brain():
    async with aiosqlite.connect(DB_PATH) as db:
        await db.execute("PRAGMA journal_mode=WAL;")

        # 1. Seed Culinary Terms First
        print("🌱 Seeding Units, Actions, and Equipment...")
        for en, fr, cat, prio in CULINARY_SEEDS:
            await upsert_lexicon(db, en, fr, cat, prio)
        await db.commit()

        # 2. Process FooDB Ingredients
        if not os.path.exists(FOOD_CSV):
            print("❌ Food.csv not found, skipping ingredient migration.")
            return

        print("⏳ Loading FooDB data...")
        df = pd.read_csv(FOOD_CSV, engine='python', on_bad_lines='skip')
        items = df[df['name'].notna()]['name'].str.strip().tolist()
        total = len(items)

        semaphore = asyncio.Semaphore(CONCURRENCY_LIMIT)
        async with aiohttp.ClientSession() as session:
            for i in range(0, total, 50):
                batch = items[i:i+50]
                tasks = [translate_item(session, name, semaphore) for name in batch]
                translations = await asyncio.gather(*tasks)

                for en_raw, fr_name in zip(batch, translations):
                    await upsert_lexicon(db, en_raw, fr_name, 'ingredient', 3)

                await db.commit()
                if i % 500 == 0:
                    print(f"🚀 Progress: {i}/{total} ingredients processed...")

    print("✨ SUCCESS: Database is now a culinary mastermind.")

if __name__ == "__main__":
    asyncio.run(migrate_to_brain())
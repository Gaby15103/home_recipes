import sqlite3
import os

DB_PATH = "dictionary.db"
SCHEMA_PATH = "up.sql"

def reset_db():
    # 1. Force remove the DB and any lock files (-journal or -wal)
    for ext in ["", "-journal", "-wal"]:
        file_to_del = DB_PATH + ext
        if os.path.exists(file_to_del):
            try:
                os.remove(file_to_del)
                print(f"🗑️ Deleted {file_to_del}")
            except Exception as e:
                print(f"⚠️ Could not delete {file_to_del}: {e}")

    # 2. Re-create and Initialize
    print(f"🏗️ Rebuilding schema from {SCHEMA_PATH}...")
    try:
        conn = sqlite3.connect(DB_PATH)
        cursor = conn.cursor()

        # Enable WAL mode immediately for better concurrency
        cursor.execute("PRAGMA journal_mode=WAL;")

        with open(SCHEMA_PATH, 'r') as f:
            cursor.executescript(f.read())

        conn.commit()
        conn.close()
        print("✨ Database is now clean and ready for a fresh migration.")
    except Exception as e:
        print(f"❌ Error rebuilding DB: {e}")

if __name__ == "__main__":
    reset_db()
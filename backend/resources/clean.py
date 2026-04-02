import os

DB_PATH = "dictionary.db"

def nuclear_wipe():
    # Extensions created by WAL mode and transactions
    targets = [DB_PATH, f"{DB_PATH}-wal", f"{DB_PATH}-shm", f"{DB_PATH}-journal"]

    print(f"🧨 Initializing total wipe of {DB_PATH}...")

    found_anything = False
    for target in targets:
        if os.path.exists(target):
            try:
                os.remove(target)
                print(f"🗑️  Removed: {target}")
                found_anything = True
            except Exception as e:
                print(f"❌ Failed to remove {target}: {e}")
                print("💡 Tip: Close any open connections in Rust or your IDE first.")

    if not found_anything:
        print("Empty: No database files were found to delete.")
    else:
        print("✨ Database successfully deleted. It no longer exists.")

if __name__ == "__main__":
    nuclear_wipe()
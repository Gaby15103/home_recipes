use anyhow::Result;
use chrono::Duration;
use std::fs;
use std::path::Path;

pub fn cleanup_logs(log_dir: &Path, keep_days: u64) -> Result<()> {
    let cutoff = chrono::Local::now() - Duration::days(keep_days as i64);

    println!("🗑️  Cleaning logs older than {} days...", keep_days);

    let mut removed_count = 0;

    for entry in fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            if let Ok(date) = chrono::NaiveDate::parse_from_str(dir_name, "%Y-%m-%d") {
                let dir_datetime = date.and_hms_opt(0, 0, 0).unwrap();

                if dir_datetime < cutoff.naive_local() {
                    fs::remove_dir_all(&path)?;
                    removed_count += 1;
                    println!("  Removed: {}", dir_name);
                }
            }
        }
    }

    println!("✅ Cleaned {} directories", removed_count);

    Ok(())
}
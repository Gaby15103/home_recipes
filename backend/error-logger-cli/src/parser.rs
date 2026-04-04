use crate::models::ErrorLog;
use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn parse_error_log(json_str: &str) -> Result<ErrorLog> {
    serde_json::from_str(json_str).map_err(|e| e.into())
}

pub fn read_log_file(path: &Path) -> Result<Vec<ErrorLog>> {
    let content = fs::read_to_string(path)?;
    let errors = content
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            parse_error_log(line).ok()
        })
        .collect();
    Ok(errors)
}

pub fn read_all_logs(log_dir: &Path) -> Result<Vec<ErrorLog>> {
    let mut all_errors = Vec::new();

    for entry in WalkDir::new(log_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "jsonl"))
    {
        match read_log_file(entry.path()) {
            Ok(errors) => all_errors.extend(errors),
            Err(e) => eprintln!("Warning: Failed to read {}: {}", entry.path().display(), e),
        }
    }

    all_errors.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(all_errors)
}

pub fn read_logs_by_date(log_dir: &Path, date: &str) -> Result<Vec<ErrorLog>> {
    let date_dir = log_dir.join(date);

    if !date_dir.exists() {
        anyhow::bail!("No logs found for date: {}", date);
    }

    let mut errors = Vec::new();
    for entry in WalkDir::new(&date_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "jsonl"))
    {
        match read_log_file(entry.path()) {
            Ok(mut day_errors) => errors.append(&mut day_errors),
            Err(e) => eprintln!("Warning: Failed to read {}: {}", entry.path().display(), e),
        }
    }

    errors.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(errors)
}
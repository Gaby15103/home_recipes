use super::models::ErrorLog;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;
use serde_json;

/// Handles error logging to file and stdout
pub struct ErrorLogger {
    log_dir: PathBuf,
    enable_console: bool,
    enable_file: bool,
}

impl ErrorLogger {
    pub fn new(log_dir: impl Into<PathBuf>) -> Self {
        let log_dir = log_dir.into();

        // Create log directory if it doesn't exist
        let _ = fs::create_dir_all(&log_dir);

        Self {
            log_dir,
            enable_console: true,
            enable_file: true,
        }
    }

    pub fn from_env() -> Self {
        let log_dir = std::env::var("ERROR_LOG_DIR")
            .unwrap_or_else(|_| "./logs/errors".to_string());

        Self::new(log_dir)
    }

    /// Log error and return the error ID for client response
    pub fn log(&self, error_log: &ErrorLog) -> String {
        let error_id = error_log.error_id.clone();

        if self.enable_console {
            self.log_to_console(error_log);
        }

        if self.enable_file {
            if let Err(e) = self.log_to_file(error_log) {
                eprintln!("Failed to write error log to file: {}", e);
            }
        }

        error_id
    }

    fn log_to_console(&self, error_log: &ErrorLog) {
        let json = serde_json::to_string_pretty(error_log)
            .unwrap_or_else(|_| format!("{:?}", error_log));

        log::error!(
            "[{}] {}: {}\n{}",
            error_log.error_id,
            error_log.error_type,
            error_log.message,
            json
        );
    }

    fn log_to_file(&self, error_log: &ErrorLog) -> std::io::Result<()> {
        let now = Local::now();
        let date_str = now.format("%Y-%m-%d").to_string();
        let hour_str = now.format("%Y%m%d_%H").to_string();

        // Create daily folder: logs/errors/2026-04-03/
        let date_dir = self.log_dir.join(&date_str);
        fs::create_dir_all(&date_dir)?;

        // Hourly log file: 2026040303_errors.jsonl
        let log_file = date_dir.join(format!("{}_errors.jsonl", hour_str));

        // Append as JSON Lines (one JSON per line for easy parsing)
        let json = serde_json::to_string(error_log)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let mut content = json;
        content.push('\n');

        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?
            .write_all(content.as_bytes())?;

        Ok(())
    }

    /// Search logs for a specific error ID (helpful for debugging)
    pub fn find_error(&self, error_id: &str) -> std::io::Result<Option<ErrorLog>> {
        for entry in fs::read_dir(&self.log_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                for file_entry in fs::read_dir(&path)? {
                    let file_entry = file_entry?;
                    let file_path = file_entry.path();

                    if file_path.extension().map_or(false, |ext| ext == "jsonl") {
                        if let Ok(content) = fs::read_to_string(&file_path) {
                            for line in content.lines() {
                                if let Ok(log) = serde_json::from_str::<ErrorLog>(line) {
                                    if log.error_id == error_id {
                                        return Ok(Some(log));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }
}
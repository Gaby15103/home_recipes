use crate::parser;
use crate::printer;
use anyhow::Result;
use std::path::Path;

pub fn find_error(log_dir: &Path, error_id: &str) -> Result<()> {
    let all_errors = parser::read_all_logs(log_dir)?;

    if let Some(error) = all_errors.iter().find(|e| e.error_id == error_id) {
        printer::print_error_detailed(error);
        Ok(())
    } else {
        println!("❌ Error not found: {}", error_id);
        Err(anyhow::anyhow!("Error ID not found"))
    }
}
use crate::parser;
use crate::printer;
use anyhow::Result;
use std::path::Path;

pub fn stats(log_dir: &Path, date: Option<String>) -> Result<()> {
    let errors = if let Some(d) = date {
        parser::read_logs_by_date(log_dir, &d)?
    } else {
        parser::read_all_logs(log_dir)?
    };

    if errors.is_empty() {
        println!("No errors found");
        return Ok(());
    }

    printer::print_statistics(&errors);
    Ok(())
}
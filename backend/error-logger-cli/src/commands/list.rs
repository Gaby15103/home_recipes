use crate::parser;
use crate::printer;
use anyhow::Result;
use std::path::Path;
use colored::Colorize;

pub fn list_errors(
    log_dir: &Path,
    error_type: Option<String>,
    date: Option<String>,
    limit: Option<usize>,
    id_contains: Option<String>,
) -> Result<()> {
    let mut errors = if let Some(d) = date {
        parser::read_logs_by_date(log_dir, &d)?
    } else {
        parser::read_all_logs(log_dir)?
    };

    // Apply filters
    if let Some(etype) = error_type {
        errors.retain(|e| e.error_type.contains(&etype));
    }

    if let Some(id_filter) = id_contains {
        errors.retain(|e| e.error_id.contains(&id_filter));
    }

    let limit = limit.unwrap_or(20);
    errors.truncate(limit);

    if errors.is_empty() {
        println!("No errors found matching criteria");
        return Ok(());
    }

    println!(
        "\n{} {} {}\n",
        "Time".cyan(),
        "Error ID".yellow(),
        "Type".magenta()
    );

    for error in &errors {
        printer::print_error_summary(error);
    }

    println!("\n📊 Showing {} errors", errors.len());

    Ok(())
}
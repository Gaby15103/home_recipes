use crate::parser;
use crate::printer;
use anyhow::Result;
use colored::*;
use std::path::Path;

pub fn analyze_parser_errors(
    log_dir: &Path,
    date: Option<String>,
    details: bool,
) -> Result<()> {
    let mut errors = if let Some(d) = date {
        parser::read_logs_by_date(log_dir, &d)?
    } else {
        parser::read_all_logs(log_dir)?
    };

    errors.retain(|e| e.parser_context.is_some());

    if errors.is_empty() {
        println!("No parser errors found");
        return Ok(());
    }

    println!(
        "\n{}",
        format!("Found {} parser errors", errors.len())
            .yellow()
            .bold()
    );

    let mut file_errors: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for error in &errors {
        if let Some(parser_ctx) = &error.parser_context {
            if let Some(file) = &parser_ctx.file_name {
                *file_errors.entry(file.clone()).or_insert(0) += 1;
            }
        }

        if details {
            printer::print_error_detailed(error);
        } else {
            printer::print_error_summary(error);
        }
    }

    println!("\n{}", "Files with most errors:".cyan().bold());
    let mut files: Vec<_> = file_errors.iter().collect();
    files.sort_by(|a, b| b.1.cmp(a.1));
    for (file, count) in files.iter().take(10) {
        println!("  {} {}", file.magenta(), count.to_string().red().bold());
    }

    Ok(())
}
use crate::parser;
use anyhow::Result;
use colored::*;
use std::collections::HashMap;
use std::path::Path;

pub fn compare_dates(
    log_dir: &Path,
    date1: &str,
    date2: &str,
) -> Result<()> {
    let errors1 = parser::read_logs_by_date(log_dir, date1)?;
    let errors2 = parser::read_logs_by_date(log_dir, date2)?;

    println!(
        "\n{}",
        format!(
            "Comparing {} ({} errors) vs {} ({} errors)",
            date1,
            errors1.len(),
            date2,
            errors2.len()
        )
            .cyan()
            .bold()
    );
    println!("{}", "═".repeat(60).cyan());

    // Compare by error type
    let mut types1: HashMap<String, usize> = HashMap::new();
    let mut types2: HashMap<String, usize> = HashMap::new();

    for error in &errors1 {
        *types1.entry(error.error_type.clone()).or_insert(0) += 1;
    }
    for error in &errors2 {
        *types2.entry(error.error_type.clone()).or_insert(0) += 1;
    }

    println!("\n{}", "Error Types:".yellow().bold());
    let all_types: std::collections::HashSet<_> =
        types1.keys().chain(types2.keys()).cloned().collect();

    for etype in all_types {
        let count1 = types1.get(&etype).copied().unwrap_or(0);
        let count2 = types2.get(&etype).copied().unwrap_or(0);
        let diff = (count2 as i32) - (count1 as i32);

        println!(
            "  {} {} → {} {}",
            etype.white(),
            count1.to_string().cyan(),
            count2.to_string().cyan(),
            if diff > 0 {
                format!("(+{})", diff).red().bold()
            } else if diff < 0 {
                format!("({})", diff).green().bold()
            } else {
                "".normal()
            }
        );
    }

    let growth = ((errors2.len() as f64 / errors1.len().max(1) as f64) - 1.0) * 100.0;
    println!(
        "\n{}",
        format!("Overall change: {:.1}%", growth)
            .bold()
            .color(if growth > 0.0 { "red" } else { "green" })
    );

    Ok(())
}
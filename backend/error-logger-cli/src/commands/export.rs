use crate::parser;
use anyhow::Result;
use std::fs::File;
use std::path::Path;

pub fn export_errors(
    log_dir: &Path,
    format: &str,
    output: &Path,
    error_type: Option<String>,
    date: Option<String>,
) -> Result<()> {
    let mut errors = if let Some(d) = date {
        parser::read_logs_by_date(log_dir, &d)?
    } else {
        parser::read_all_logs(log_dir)?
    };

    if let Some(etype) = error_type {
        errors.retain(|e| e.error_type.contains(&etype));
    }

    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&errors)?;
            std::fs::write(output, json)?;
        }
        "csv" => {
            let mut wtr = csv::Writer::from_writer(File::create(output)?);
            for error in &errors {
                wtr.serialize((
                    &error.error_id,
                    &error.error_type,
                    &error.timestamp.to_rfc3339(),
                    &error.message,
                ))?;
            }
            wtr.flush()?;
        }
        _ => anyhow::bail!("Unknown format: {}", format),
    }

    println!(
        "✅ Exported {} errors to {}",
        errors.len(),
        output.display()
    );

    Ok(())
}
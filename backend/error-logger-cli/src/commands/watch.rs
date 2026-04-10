use crate::parser;
use crate::printer;
use anyhow::Result;
use std::path::Path;

pub fn watch_logs(log_dir: &Path, interval: u64) -> Result<()> {
    let mut last_count = 0;

    println!("👀 Watching for new errors... (Ctrl+C to stop)\n");

    loop {
        match parser::read_all_logs(log_dir) {
            Ok(errors) => {
                if errors.len() > last_count {
                    let new_errors = &errors[..(errors.len() - last_count)];
                    println!(
                        "🔴 {} new error(s) detected:",
                        new_errors.len()
                    );
                    for error in new_errors.iter().take(5) {
                        printer::print_error_summary(error);
                    }
                    if new_errors.len() > 5 {
                        println!("   ... and {} more", new_errors.len() - 5);
                    }
                    println!();
                }
                last_count = errors.len();
            }
            Err(e) => eprintln!("Error reading logs: {}", e),
        }
        
    }
}
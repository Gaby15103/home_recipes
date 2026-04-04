use crate::models::ErrorLog;
use colored::*;
use serde_json::to_string_pretty;
use std::collections::HashMap;

pub fn print_error_detailed(error: &ErrorLog) {
    println!("\n{}", "═".repeat(80).red().bold());
    println!(
        "{} {}",
        "Error ID:".cyan().bold(),
        error.error_id.yellow()
    );
    println!(
        "{} {}",
        "Type:".cyan().bold(),
        format_error_type(&error.error_type)
    );
    println!(
        "{} {}",
        "Time:".cyan().bold(),
        error.timestamp.to_rfc3339().white()
    );
    println!(
        "{} {}",
        "Message:".cyan().bold(),
        error.message.white()
    );

    println!(
        "\n{}",
        "Environment:".cyan().bold()
    );
    println!("  {} {}", "Environment:".white(), error.environment.environment.yellow());
    println!("  {} {}", "Host:".white(), error.environment.host.yellow());
    println!("  {} {}", "App Version:".white(), error.environment.app_version.yellow());
    println!("  {} {}", "Rust Version:".white(), error.environment.rust_version.white());

    if let Some(http) = &error.http_context {
        println!("\n{}", "HTTP Context:".cyan().bold());
        println!("  {} {}", "Method:".white(), http.method.yellow());
        println!("  {} {}", "Path:".white(), http.path.yellow());
        println!("  {} {}", "Status:".white(), http.status_code.to_string().red());
        if let Some(user_id) = &http.user_id {
            println!("  {} {}", "User ID:".white(), user_id.yellow());
        }
    }

    if let Some(parser) = &error.parser_context {
        println!("\n{}", "Parser Context:".cyan().bold());
        println!("  {} {}", "Parser Type:".white(), parser.parser_type.yellow());
        println!("  {} {}", "Stage:".white(), parser.parse_stage.red());
        if let Some(file) = &parser.file_name {
            println!("  {} {}", "File:".white(), file.yellow());
        }
        if let Some(size) = parser.file_size {
            println!("  {} {} bytes", "Size:".white(), size.to_string().yellow());
        }
        if let Some(sample) = &parser.input_sample {
            println!("  {} {}", "Sample (first 200 chars):".white(),
                     format!("{}...", &sample[..sample.len().min(200)]).dimmed());
        }
    }

    if let Some(db) = &error.db_context {
        println!("\n{}", "Database Context:".cyan().bold());
        println!("  {} {}", "Operation:".white(), db.operation.yellow());
        println!("  {} {}", "Table:".white(), db.table.yellow());
        println!("  {} {}", "Error:".white(), db.error_msg.red());
    }

    if let Some(stack) = &error.stack_trace {
        println!("\n{}", "Stack Trace:".cyan().bold());
        println!("{}", stack.dimmed());
    }

    if !error.metadata.is_empty() {
        println!("\n{}", "Metadata:".cyan().bold());
        for (k, v) in &error.metadata {
            println!("  {} {}", format!("{}:", k).white(), v.yellow());
        }
    }

    println!("{}", "═".repeat(80).red().bold());
}

pub fn print_error_summary(error: &ErrorLog) {
    println!(
        "{} {} | {} | {} | {}",
        error.timestamp.format("%Y-%m-%d %H:%M:%S").to_string().cyan(),
        error.error_id.yellow(),
        format_error_type(&error.error_type),
        truncate(&error.message, 50).white(),
        error
            .parser_context
            .as_ref()
            .and_then(|p| p.file_name.as_ref())
            .map(|f| f.as_str())
            .unwrap_or("—")
            .dimmed()
    );
}

pub fn print_statistics(errors: &[ErrorLog]) {
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    let mut env_counts: HashMap<String, usize> = HashMap::new();
    let mut stage_counts: HashMap<String, usize> = HashMap::new();

    for error in errors {
        *type_counts.entry(error.error_type.clone()).or_insert(0) += 1;
        *env_counts.entry(error.environment.environment.clone()).or_insert(0) += 1;

        if let Some(parser) = &error.parser_context {
            *stage_counts
                .entry(parser.parse_stage.clone())
                .or_insert(0) += 1;
        }
    }

    println!("\n{}", "Error Statistics".cyan().bold());
    println!("{}", "═".repeat(50).cyan());

    println!("\n{}", "Errors by Type:".white().bold());
    let mut types: Vec<_> = type_counts.iter().collect();
    types.sort_by(|a, b| b.1.cmp(a.1));
    for (error_type, count) in types {
        println!("  {} {}", error_type.yellow(), count.to_string().red().bold());
    }

    println!("\n{}", "Errors by Environment:".white().bold());
    let mut envs: Vec<_> = env_counts.iter().collect();
    envs.sort_by(|a, b| b.1.cmp(a.1));
    for (env, count) in envs {
        println!("  {} {}", env.cyan(), count.to_string().white().bold());
    }

    if !stage_counts.is_empty() {
        println!("\n{}", "Parser Errors by Stage:".white().bold());
        let mut stages: Vec<_> = stage_counts.iter().collect();
        stages.sort_by(|a, b| b.1.cmp(a.1));
        for (stage, count) in stages {
            println!("  {} {}", stage.magenta(), count.to_string().white().bold());
        }
    }

    println!("\n{}", format!("Total Errors: {}", errors.len()).green().bold());
}

fn format_error_type(error_type: &str) -> ColoredString {
    match error_type {
        "ParserError" => error_type.red().bold(),
        "DatabaseError" => error_type.magenta().bold(),
        "InternalServerError" => error_type.red().bold(),
        _ => error_type.yellow(),
    }
}

fn truncate(s: &str, len: usize) -> String {
    if s.len() > len {
        format!("{}...", &s[..len])
    } else {
        s.to_string()
    }
}

pub fn print_json(error: &ErrorLog) {
    if let Ok(json) = to_string_pretty(error) {
        println!("{}", json);
    }
}
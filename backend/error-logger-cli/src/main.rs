use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod parser;
mod printer;
mod models;

use commands::*;

#[derive(Parser)]
#[command(
    name = "error-logs",
    about = "CLI tool for debugging error logs from the home_recipes backend",
    long_about = "Parse, search, and analyze error logs to debug production issues"
)]
struct Cli {
    /// Path to the error logs directory (default: ./logs/errors)
    #[arg(global = true, short, long)]
    log_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for a specific error by ID
    Find {
        /// Error ID to search for
        error_id: String,
    },

    /// List all errors (optionally filtered)
    List {
        /// Filter by error type (e.g., "ParserError", "DatabaseError")
        #[arg(short, long)]
        error_type: Option<String>,

        /// Filter by date (YYYY-MM-DD)
        #[arg(short, long)]
        date: Option<String>,

        /// Show only the last N errors
        #[arg(short, long)]
        limit: Option<usize>,

        /// Filter by error ID prefix
        #[arg(long)]
        id_contains: Option<String>,
    },

    /// Show statistics about errors
    Stats {
        /// Filter by date (YYYY-MM-DD)
        #[arg(short, long)]
        date: Option<String>,
    },

    /// Search errors by content pattern
    Search {
        /// Regex pattern to search for
        pattern: String,

        /// Search field: message, parser_context, db_context, stack_trace, all
        #[arg(short, long, default_value = "all")]
        field: String,

        /// Show only matching field
        #[arg(short, long)]
        extract: bool,
    },

    /// Export errors to CSV or JSON
    Export {
        /// Output format: csv, json
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Filter by error type
        #[arg(short, long)]
        error_type: Option<String>,

        /// Filter by date
        #[arg(short, long)]
        date: Option<String>,
    },

    /// Analyze parser errors specifically
    Parser {
        /// Filter by date
        #[arg(short, long)]
        date: Option<String>,

        /// Show detailed parser context
        #[arg(short, long)]
        details: bool,
    },

    /// Compare errors between two dates
    Compare {
        /// First date (YYYY-MM-DD)
        date1: String,

        /// Second date (YYYY-MM-DD)
        date2: String,
    },

    /// Watch log directory for new errors (live mode)
    Watch {
        /// Refresh interval in seconds
        #[arg(short, long, default_value = "2")]
        interval: u64,
    },

    /// Clear old logs (older than N days)
    Cleanup {
        /// Number of days to keep
        #[arg(short, long, default_value = "7")]
        keep_days: u64,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let log_dir = cli.log_dir.unwrap_or_else(|| {
        std::env::var("ERROR_LOG_DIR")
            .unwrap_or_else(|_| "./logs/errors".to_string())
            .into()
    });

    match cli.command {
        Commands::Find { error_id } => {
            find_error(&log_dir, &error_id)?
        }
        Commands::List {
            error_type,
            date,
            limit,
            id_contains,
        } => {
            list_errors(&log_dir, error_type, date, limit, id_contains)?
        }
        Commands::Stats { date } => {
            stats(&log_dir, date)?
        }
        Commands::Search {
            pattern,
            field,
            extract,
        } => {
            search_errors(&log_dir, &pattern, &field, extract)?
        }
        Commands::Export {
            format,
            output,
            error_type,
            date,
        } => {
            export_errors(&log_dir, &format, &output, error_type, date)?
        }
        Commands::Parser { date, details } => {
            analyze_parser_errors(&log_dir, date, details)?
        }
        Commands::Compare { date1, date2 } => {
            compare_dates(&log_dir, &date1, &date2)?
        }
        Commands::Watch { interval } => {
            watch_logs(&log_dir, interval)?
        }
        Commands::Cleanup { keep_days } => {
            cleanup_logs(&log_dir, keep_days)?
        }
    }

    Ok(())
}
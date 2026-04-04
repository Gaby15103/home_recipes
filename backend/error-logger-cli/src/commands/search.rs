use crate::parser;
use crate::printer;
use anyhow::Result;
use regex::Regex;
use std::path::Path;

pub fn search_errors(
    log_dir: &Path,
    pattern: &str,
    field: &str,
    extract: bool,
) -> Result<()> {
    let re = Regex::new(pattern)?;
    let all_errors = parser::read_all_logs(log_dir)?;

    let mut matches = Vec::new();

    for error in all_errors {
        let mut matched = false;
        let mut matched_content = String::new();

        if field == "all" || field == "message" {
            if re.is_match(&error.message) {
                matched = true;
                matched_content = error.message.clone();
            }
        }

        if (field == "all" || field == "parser_context") && !matched {
            if let Some(parser_ctx) = &error.parser_context {
                let ctx_str = serde_json::to_string(parser_ctx).unwrap_or_default();
                if re.is_match(&ctx_str) {
                    matched = true;
                    matched_content = ctx_str;
                }
            }
        }

        if (field == "all" || field == "stack_trace") && !matched {
            if let Some(stack) = &error.stack_trace {
                if re.is_match(stack) {
                    matched = true;
                    matched_content = stack.clone();
                }
            }
        }

        if matched {
            matches.push((error, matched_content));
        }
    }

    if matches.is_empty() {
        println!("❌ No matches found for pattern: {}", pattern);
        return Ok(());
    }

    println!("✅ Found {} matches\n", matches.len());

    for (error, content) in matches {
        if extract {
            println!("{}", content);
        } else {
            printer::print_error_summary(&error);
        }
    }

    Ok(())
}
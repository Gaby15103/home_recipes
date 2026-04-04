# Find a specific error
cargo run --bin error-logs -- find abc123def456

# List all errors (last 20)
```bash
cargo run --bin error-logs -- list
```
# Show all parser errors from today
```bash
cargo run --bin error-logs -- list --error-type ParserError
```
# Get statistics for a specific date
```bash
cargo run --bin error-logs -- stats --date 2026-04-03
```
# Search for patterns
```bash
cargo run --bin error-logs -- search "recipe" --field parser_context
```
# Export to JSON
```bash
cargo run --bin error-logs -- export --format json --output errors.json
```
# Analyze parser errors with details
```bash
cargo run --bin error-logs -- parser --details
```
# Compare two dates
```bash
cargo run --bin error-logs -- compare 2026-04-02 2026-04-03
```
# Watch for new errors (live)
```bash
cargo run --bin error-logs -- watch --interval 2
```
# Clean old logs (keep last 7 days)
```bash
cargo run --bin error-logs -- cleanup --keep-days 7
```
# Custom log directory
```bash
cargo run --bin error-logs -- --log-dir /var/logs/myapp find abc123
```
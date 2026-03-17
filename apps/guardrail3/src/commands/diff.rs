use std::path::Path;

use crate::commands::generate;

/// Full diff output (for `rs diff` / `ts diff` commands — detailed view).
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run(path: &str) {
    let project_path = Path::new(path);

    let Some(expected) = generate::generate_expected(project_path) else {
        eprintln!("Error: guardrail3.toml not found or invalid at {path}");
        std::process::exit(1);
    };

    let has_diff = show_diff_summary(&expected, project_path);
    if has_diff {
        std::process::exit(1);
    } else {
        println!("No changes. All generated files are current.");
    }
}

#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run_ts(path: &str) {
    let project_path = Path::new(path);

    let Some(expected) = generate::generate_expected_ts(project_path) else {
        eprintln!("Error: guardrail3.toml not found or invalid at {path}");
        std::process::exit(1);
    };

    let has_diff = show_diff_summary(&expected, project_path);
    if has_diff {
        std::process::exit(1);
    } else {
        println!("No changes. All generated TypeScript files are current.");
    }
}

/// Show a concise summary of what generate would change.
/// New files: name + line count. Changed files: name + changed line count.
#[allow(clippy::print_stdout)] // reason: CLI command — diff output to stdout
#[allow(clippy::type_complexity)] // reason: slice of tuples from generate_expected
fn show_diff_summary(expected: &[(String, String)], project_path: &Path) -> bool {
    let mut has_diff = false;

    for (rel_path, expected_content) in expected {
        let full_path = project_path.join(rel_path);
        match crate::fs::read_file_err(&full_path) {
            Ok(actual) => {
                if actual != *expected_content {
                    has_diff = true;
                    let changed = count_changed_lines(&actual, expected_content);
                    println!("  Would update {rel_path} ({changed} lines changed)");
                }
            }
            Err(_) => {
                has_diff = true;
                let lines = expected_content.lines().count();
                println!("  Would create {rel_path} ({lines} lines)");
            }
        }
    }

    has_diff
}

fn count_changed_lines(actual: &str, expected: &str) -> usize {
    let actual_lines: Vec<&str> = actual.lines().collect();
    let expected_lines: Vec<&str> = expected.lines().collect();
    let max_len = actual_lines.len().max(expected_lines.len());
    let mut changed = 0usize;

    for i in 0..max_len {
        let a = actual_lines.get(i).copied();
        let b = expected_lines.get(i).copied();
        if a != b {
            changed = changed.saturating_add(1);
        }
    }

    changed
}

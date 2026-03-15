use std::path::Path;

use crate::commands::generate;

#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run(path: &str) {
    let project_path = Path::new(path);

    let Some(expected) = generate::generate_expected(project_path) else {
        eprintln!("Error: guardrail3.toml not found or invalid at {path}");
        std::process::exit(1);
    };

    let mut has_diff = false;

    for (rel_path, expected_content) in &expected {
        let full_path = project_path.join(rel_path);
        match crate::fs::read_file_err(&full_path) {
            Ok(actual) => {
                if actual != *expected_content {
                    has_diff = true;
                    println!("--- {rel_path} (current)");
                    println!("+++ {rel_path} (expected)");
                    print_simple_diff(&actual, expected_content);
                    println!();
                }
            }
            Err(_) => {
                has_diff = true;
                println!("--- /dev/null");
                println!("+++ {rel_path} (new file)");
                for line in expected_content.lines() {
                    println!("+{line}");
                }
                println!();
            }
        }
    }

    if has_diff {
        std::process::exit(1);
    } else {
        println!("No changes. All generated files are current.");
    }
}

#[allow(clippy::print_stdout)] // reason: CLI command — diff output to stdout
fn print_simple_diff(actual: &str, expected: &str) {
    let actual_lines: Vec<&str> = actual.lines().collect();
    let expected_lines: Vec<&str> = expected.lines().collect();

    let max_len = actual_lines.len().max(expected_lines.len());
    let mut i = 0usize;

    while i < max_len {
        let a = actual_lines.get(i).copied();
        let b = expected_lines.get(i).copied();

        match (a, b) {
            (Some(al), Some(bl)) => {
                if al == bl {
                    println!(" {al}");
                } else {
                    println!("-{al}");
                    println!("+{bl}");
                }
            }
            (Some(al), None) => {
                println!("-{al}");
            }
            (None, Some(bl)) => {
                println!("+{bl}");
            }
            (None, None) => {}
        }

        i = i.saturating_add(1);
    }
}

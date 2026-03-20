use std::path::Path;

use crate::adapters::inbound::cli::generate;

#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run(path: &str) {
    let project_path = Path::new(path);

    let Some(expected) = generate::generate_expected(project_path) else {
        eprintln!("Error: guardrail3.toml not found or invalid at {path}");
        std::process::exit(1);
    };

    let mut stale_count = 0usize;

    for (rel_path, expected_content) in &expected {
        let full_path = project_path.join(rel_path);
        match crate::fs::read_file_err(&full_path) {
            Ok(actual) => {
                if actual != *expected_content {
                    println!("STALE: {rel_path}");
                    stale_count = stale_count.saturating_add(1);
                }
            }
            Err(_) => {
                println!("MISSING: {rel_path}");
                stale_count = stale_count.saturating_add(1);
            }
        }
    }

    if stale_count == 0 {
        println!("All generated files are current.");
    } else {
        println!();
        println!("{stale_count} file(s) need regeneration. Run: guardrail3 generate");
        std::process::exit(1);
    }
}

use std::path::Path;

use crate::commands::generate;

/// Dry-run for rs generate — shows what would be enforced.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run(path: &str) {
    let project_path = Path::new(path);

    let Some(expected) = generate::generate_expected(project_path) else {
        eprintln!("Error: guardrail3.toml not found or invalid at {path}");
        std::process::exit(1);
    };

    let has_diff = has_changes(&expected, project_path);
    if !has_diff {
        println!("No changes. All generated files are current.");
        return;
    }

    println!("Would configure:\n");
    describe_rs_config(&expected);

    std::process::exit(1);
}

/// Dry-run for ts generate.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run_ts(path: &str) {
    let project_path = Path::new(path);

    let Some(expected) = generate::generate_expected_ts(project_path) else {
        eprintln!("Error: guardrail3.toml not found or invalid at {path}");
        std::process::exit(1);
    };

    let has_diff = has_changes(&expected, project_path);
    if !has_diff {
        println!("No changes. All generated TypeScript files are current.");
        return;
    }

    println!("Would configure:\n");
    describe_ts_config(&expected);

    std::process::exit(1);
}

#[allow(clippy::type_complexity)] // reason: slice of tuples from generate_expected
fn has_changes(expected: &[(String, String)], project_path: &Path) -> bool {
    expected.iter().any(|(rel_path, content)| {
        let full = project_path.join(rel_path);
        crate::fs::read_file_err(&full).map_or(true, |actual| actual != *content)
    })
}

/// Describe what Rust config files would enforce.
#[allow(clippy::print_stdout)] // reason: CLI dry-run output
#[allow(clippy::type_complexity)] // reason: slice of tuples from generate_expected
fn describe_rs_config(expected: &[(String, String)]) {
    for (rel_path, content) in expected {
        if rel_path.ends_with("clippy.toml") {
            let in_methods = count_section_entries(content, "disallowed-methods");
            let in_types = count_section_entries(content, "disallowed-types");

            if rel_path.contains('/') {
                // Per-crate clippy.toml
                let crate_name = rel_path.split('/').next().unwrap_or(rel_path);
                println!(
                    "  Clippy bans for {crate_name}: {in_methods} method bans, {in_types} type bans"
                );
            } else {
                println!("  Clippy bans: {in_methods} method bans, {in_types} type bans");
            }

            // Summarize ban categories
            let categories = describe_ban_categories(content);
            if !categories.is_empty() {
                println!("    {categories}");
            }
        } else if rel_path.ends_with("deny.toml") {
            let bans = content.matches("{ name =").count();
            let feature_bans = content.matches("[[bans.features]]").count();
            println!("  Dependency bans: {bans} crate bans, {feature_bans} feature bans");
        } else if rel_path.ends_with("rustfmt.toml") {
            println!("  Code style: rustfmt (auto-formatting)");
        } else if rel_path.ends_with("rust-toolchain.toml") {
            let channel = content
                .lines()
                .find(|l| l.starts_with("channel"))
                .unwrap_or("stable");
            println!("  Toolchain: {channel}");
        } else if rel_path.ends_with("release-plz.toml") {
            println!("  Release: release-plz (automated publishing)");
        } else if rel_path.ends_with("cliff.toml") {
            println!("  Changelog: git-cliff (conventional commits)");
        } else if rel_path.ends_with("pre-commit") {
            let steps = describe_hook_steps(content);
            println!("  Pre-commit hook:");
            println!("    {steps}");
        }
    }
}

/// Describe what TS config files would enforce.
#[allow(clippy::print_stdout)] // reason: CLI dry-run output
#[allow(clippy::type_complexity)] // reason: slice of tuples from generate_expected
fn describe_ts_config(expected: &[(String, String)]) {
    for (rel_path, content) in expected {
        if rel_path.ends_with("pre-commit") {
            let steps = describe_hook_steps(content);
            println!("  Pre-commit hook:");
            println!("    {steps}");
        } else {
            let lines = content.lines().count();
            println!("  {rel_path} ({lines} lines)");
        }
    }
}

/// Count entries in a clippy.toml section.
fn count_section_entries(content: &str, section: &str) -> usize {
    let mut in_section = false;
    let mut count = 0usize;
    for line in content.lines() {
        let trimmed = line.trim();
        let section_header = format!("{section} = [");
        if trimmed.starts_with(&section_header) || trimmed == section_header {
            in_section = true;
            continue;
        }
        if in_section {
            if trimmed == "]" {
                break;
            }
            if trimmed.starts_with("{ path =") || trimmed.starts_with("{path =") {
                count = count.saturating_add(1);
            }
        }
    }
    count
}

/// Summarize what kinds of bans are in a clippy.toml.
fn describe_ban_categories(content: &str) -> String {
    let mut cats = Vec::new();
    if content.contains("std::env::var") {
        cats.push("env vars");
    }
    if content.contains("std::fs::") {
        cats.push("filesystem");
    }
    if content.contains("process::") {
        cats.push("process control");
    }
    if content.contains("thread::sleep") {
        cats.push("blocking sleep");
    }
    if content.contains("reqwest::Client") {
        cats.push("HTTP client construction");
    }
    if content.contains("serde_json::from_") {
        cats.push("raw deserialization (use Validated<T>)");
    }
    if content.contains("HashMap") {
        cats.push("HashMap→BTreeMap");
    }
    if content.contains("Mutex") {
        cats.push("Mutex→parking_lot");
    }
    if content.contains("axum::") {
        cats.push("raw axum extractors");
    }
    cats.join(", ")
}

/// Describe hook steps from pre-commit script content.
fn describe_hook_steps(content: &str) -> String {
    let mut steps = Vec::new();
    if content.contains("conflict marker") {
        steps.push("conflict markers");
    }
    if content.contains("gitleaks") {
        steps.push("secrets");
    }
    if content.contains("guardrail3") {
        steps.push("guardrail3 validate");
    }
    if content.contains("frozen-lockfile") {
        steps.push("lockfile");
    }
    if content.contains("prettier") {
        steps.push("prettier");
    }
    if content.contains("eslint") {
        steps.push("ESLint");
    }
    if content.contains("cspell") {
        steps.push("cspell");
    }
    if content.contains("pnpm audit") {
        steps.push("pnpm audit");
    }
    if content.contains("cargo fmt") {
        steps.push("cargo fmt");
    }
    if content.contains("cargo clippy") {
        steps.push("clippy");
    }
    if content.contains("cargo deny") {
        steps.push("cargo-deny");
    }
    if content.contains("cargo machete") {
        steps.push("machete");
    }
    if content.contains("cargo test") {
        steps.push("test");
    }
    if content.contains("cargo dupes") {
        steps.push("dupes");
    }
    if content.contains("stylelint") {
        steps.push("stylelint");
    }
    steps.join(" → ")
}

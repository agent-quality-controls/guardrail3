use std::collections::BTreeSet;
use std::path::Path;

use crate::commands::generate;

/// Status of a generated file compared to what exists on disk.
enum FileStatus {
    /// File does not exist yet.
    WouldCreate,
    /// File exists and matches the generated output.
    NoChanges,
    /// File exists but differs, with no custom entries detected.
    WouldUpdate { diff_lines: usize },
    /// File exists, differs, and contains custom entries not in the generated base.
    WouldUpdateWithCustom {
        diff_lines: usize,
        custom_entries: Vec<String>,
    },
}

/// Dry-run for rs generate — shows what would change per file.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run(path: &str) {
    let project_path = Path::new(path);

    let Some(expected) = generate::generate_expected(project_path) else {
        eprintln!("Error: guardrail3.toml not found or invalid at {path}");
        std::process::exit(1);
    };

    show_smart_diff(&expected, project_path);
}

/// Dry-run for ts generate — shows what would change per file.
#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn run_ts(path: &str) {
    let project_path = Path::new(path);

    let Some(expected) = generate::generate_expected_ts(project_path) else {
        eprintln!("Error: guardrail3.toml not found or invalid at {path}");
        std::process::exit(1);
    };

    show_smart_diff(&expected, project_path);
}

/// Compare each expected file against disk and print per-file status.
///
/// Groups output by status: creates first, then updates (with custom entry
/// details for TOML files), then unchanged files.
#[allow(clippy::print_stdout)] // reason: CLI dry-run output
#[allow(clippy::type_complexity)] // reason: slice of tuples from generate_expected
fn show_smart_diff(expected: &[(String, String)], project_path: &Path) {
    let mut creates: Vec<&str> = Vec::new();
    let mut updates: Vec<(&str, usize, Vec<String>)> = Vec::new();
    let mut unchanged: Vec<&str> = Vec::new();

    for (rel_path, gen_content) in expected {
        let status = classify_file(project_path, rel_path, gen_content);
        match status {
            FileStatus::WouldCreate => creates.push(rel_path),
            FileStatus::NoChanges => unchanged.push(rel_path),
            FileStatus::WouldUpdate { diff_lines } => {
                updates.push((rel_path, diff_lines, Vec::new()));
            }
            FileStatus::WouldUpdateWithCustom {
                diff_lines,
                custom_entries,
            } => {
                updates.push((rel_path, diff_lines, custom_entries));
            }
        }
    }

    if creates.is_empty() && updates.is_empty() {
        println!("No changes needed. All generated files are current.");
        return;
    }

    for rel_path in &creates {
        println!("{rel_path} — would create");
    }

    for (rel_path, diff_lines, customs) in &updates {
        println!("{rel_path} — would update ({diff_lines} lines differ)");
        if !customs.is_empty() {
            println!("  Custom entries found — would extract to .guardrail3/overrides/:");
            for entry in customs {
                println!("    {entry}");
            }
        }
    }

    for rel_path in &unchanged {
        println!("{rel_path} — no changes needed");
    }

    println!();
    let total = creates.len().saturating_add(updates.len());
    println!(
        "{total} file(s) would change, {} file(s) up to date.",
        unchanged.len()
    );

    #[allow(clippy::disallowed_methods)] // reason: non-zero exit when changes pending
    std::process::exit(1);
}

/// Classify a single file: create, update, update-with-custom, or no-change.
fn classify_file(project_path: &Path, rel_path: &str, gen_content: &str) -> FileStatus {
    let full_path = project_path.join(rel_path);
    let Ok(actual) = crate::fs::read_file_err(&full_path) else {
        return FileStatus::WouldCreate;
    };

    if actual == gen_content {
        return FileStatus::NoChanges;
    }

    let diff_lines = count_diff_lines(&actual, gen_content);

    if is_entry_based_toml(rel_path) {
        let custom = extract_custom_entries(&actual, gen_content);
        if !custom.is_empty() {
            return FileStatus::WouldUpdateWithCustom {
                diff_lines,
                custom_entries: custom,
            };
        }
    }

    FileStatus::WouldUpdate { diff_lines }
}

/// Count lines that differ between two strings (symmetric difference of lines).
fn count_diff_lines(actual: &str, expected: &str) -> usize {
    let actual_lines: Vec<&str> = actual.lines().collect();
    let expected_lines: Vec<&str> = expected.lines().collect();
    let max_len = actual_lines.len().max(expected_lines.len());

    let mut diff_count = 0usize;
    for i in 0..max_len {
        let a = actual_lines.get(i).copied();
        let b = expected_lines.get(i).copied();
        if a != b {
            diff_count = diff_count.saturating_add(1);
        }
    }
    diff_count
}

/// Whether this file is a TOML config where we can detect custom entries
/// by comparing `{ path = ... }` or `{ name = ... }` lines.
fn is_entry_based_toml(rel_path: &str) -> bool {
    rel_path.ends_with("clippy.toml") || rel_path.ends_with("deny.toml")
}

/// Extract entries from `actual` that are NOT present in `generated`.
///
/// Looks for lines matching `{ path = ...}` (clippy.toml) and `{ name = ...}`
/// (deny.toml). Any such entry in actual that does not appear in generated is
/// a custom user entry.
fn extract_custom_entries(actual: &str, generated: &str) -> Vec<String> {
    let gen_entries = collect_toml_entries(generated);
    let actual_entries = collect_toml_entries(actual);

    let mut custom = Vec::new();
    for entry in &actual_entries {
        if !gen_entries.contains(entry) {
            custom.push(entry.clone());
        }
    }
    custom
}

/// Collect all `{ path = ... }` and `{ name = ... }` entry lines from TOML content.
///
/// Handles multiline entries (joins continuation lines until `}` is found),
/// no-space syntax (`{path=`), and section-aware keying so that identical
/// entries in different TOML sections (e.g. `disallowed-methods` vs
/// `disallowed-types`) are not incorrectly deduplicated.
fn collect_toml_entries(content: &str) -> BTreeSet<String> {
    let mut entries = BTreeSet::new();
    let mut current_section = String::new();
    let mut lines = content.lines();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        // Detect section headers
        if trimmed.contains("disallowed-methods") && trimmed.contains('[') {
            "methods".clone_into(&mut current_section);
            continue;
        }
        if trimmed.contains("disallowed-types") && trimmed.contains('[') {
            "types".clone_into(&mut current_section);
            continue;
        }
        if trimmed.contains("deny") && trimmed.contains('[') {
            "deny".clone_into(&mut current_section);
            continue;
        }
        if trimmed == "]" {
            current_section.clear();
            continue;
        }

        // Check if this is an entry line (space-insensitive prefix match)
        let normalized = trimmed.replace(' ', "");
        let is_entry = normalized.starts_with("{path=") || normalized.starts_with("{name=");
        if !is_entry {
            continue;
        }

        // Handle multiline: if line doesn't contain '}', join with next lines
        let mut full_entry = trimmed.to_owned();
        if !full_entry.contains('}') {
            for next in lines.by_ref() {
                let next_trimmed = next.trim();
                full_entry.push(' ');
                full_entry.push_str(next_trimmed);
                if next_trimmed.contains('}') {
                    break;
                }
            }
        }

        // Normalize: strip trailing comma, trim
        let clean = full_entry.trim_end_matches(',').trim().to_owned();

        // Prefix with section for section-aware comparison
        let key = if current_section.is_empty() {
            clean
        } else {
            format!("{current_section}:{clean}")
        };

        let _ = entries.insert(key);
    }
    entries
}

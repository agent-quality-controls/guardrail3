use std::path::Path;

use crate::domain::report::{CheckResult, Severity};

use super::ast_helpers;
use super::source_scan::filter_non_comment_lines;
use crate::ports::outbound::FileSystem;

// R38: File line count (>500 effective lines = error)
pub fn check_file_length(
    path: &Path,
    content: &str,
    is_test: bool,
    results: &mut Vec<CheckResult>,
) {
    // Test files are exempt from file length checks
    if is_test {
        return;
    }

    let effective_lines = filter_non_comment_lines(content).len();

    if effective_lines > 500 {
        results.push(CheckResult {
            id: "R38".to_owned(),
            severity: Severity::Error,
            title: "File too long".to_owned(),
            message: format!("{effective_lines} effective lines (max 500). Long files are hard to understand, review, and maintain. Extract helper functions into submodules or split the file by responsibility."),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

// R40-R41: use statement count
pub fn check_use_count(path: &Path, content: &str, is_test: bool, results: &mut Vec<CheckResult>) {
    // Test files are exempt from use-count checks
    if is_test {
        return;
    }

    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };
    let use_count = ast_helpers::count_use_statements(&file);

    if use_count > 20 {
        results.push(CheckResult {
            id: "R40".to_owned(),
            severity: Severity::Error,
            title: "Too many use statements".to_owned(),
            message: format!("{use_count} `use` import statements (max 20). Too many imports indicate the file has too many responsibilities or dependencies. Consolidate with `use crate::{{a, b, c}};` or split the file into smaller modules."),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });
    } else if use_count > 15 {
        results.push(CheckResult {
            id: "R41".to_owned(),
            severity: Severity::Warn,
            title: "Many use statements".to_owned(),
            message: format!("{use_count} `use` import statements (warn at 15, max 20). Too many imports indicate the file has too many responsibilities. Consolidate with `use crate::{{a, b, c}};` or split into smaller modules."),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

// R53: unsafe_code = "forbid"
pub fn check_unsafe_code_forbid(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    results: &mut Vec<CheckResult>,
) {
    let cargo_path = workspace_root.join("Cargo.toml");
    if !cargo_path.exists() {
        return;
    }

    let Some(content) = fs.read_file(&cargo_path) else {
        return;
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    let level = table
        .get("workspace")
        .and_then(|w| w.get("lints"))
        .and_then(|l| l.get("rust"))
        .and_then(|r| r.get("unsafe_code"));

    match level {
        Some(toml::Value::String(s)) if s == "forbid" => {
            results.push(CheckResult {
                id: "R53".to_owned(),
                severity: Severity::Info,
                title: "unsafe_code = forbid".to_owned(),
                message: "unsafe_code is set to \"forbid\" in workspace lints — cannot be overridden per-crate with #[allow(unsafe_code)]. This is the strongest safety setting. No action needed.".to_owned(),
                file: Some(cargo_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        }
        Some(toml::Value::String(s)) if s == "deny" => {
            results.push(CheckResult {
                id: "R53".to_owned(),
                severity: Severity::Error,
                title: "unsafe_code should be forbid".to_owned(),
                message: "unsafe_code = \"deny\" can be overridden per-crate with #[allow(unsafe_code)], bypassing the safety guarantee. Change to `unsafe_code = \"forbid\"` in [workspace.lints.rust] in Cargo.toml."
                    .to_owned(),
                file: Some(cargo_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
        _ => {
            // Already covered by R26 lint checks
        }
    }
}

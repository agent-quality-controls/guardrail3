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
            message: format!("{effective_lines} effective lines (max 500)"),
            file: Some(path.display().to_string()),
            line: None,
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
            message: format!("{use_count} use statements (max 20)"),
            file: Some(path.display().to_string()),
            line: None,
        });
    } else if use_count > 15 {
        results.push(CheckResult {
            id: "R41".to_owned(),
            severity: Severity::Warn,
            title: "Many use statements".to_owned(),
            message: format!("{use_count} use statements (warn at 15, max 20)"),
            file: Some(path.display().to_string()),
            line: None,
        });
    }
}

// R42: unsafe
pub fn check_unsafe(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };
    // AST path — no false positives from strings or comments
    for line in ast_helpers::find_unsafe_usage(&file) {
        let message = content
            .lines()
            .nth(line.saturating_sub(1))
            .unwrap_or("")
            .trim();
        results.push(CheckResult {
            id: "R42".to_owned(),
            severity: Severity::Error,
            title: "unsafe usage".to_owned(),
            message: message.to_owned(),
            file: Some(path.display().to_string()),
            line: Some(line),
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
                message: "unsafe_code is forbidden (cannot be overridden per-crate)".to_owned(),
                file: Some(cargo_path.display().to_string()),
                line: None,
            });
        }
        Some(toml::Value::String(s)) if s == "deny" => {
            results.push(CheckResult {
                id: "R53".to_owned(),
                severity: Severity::Error,
                title: "unsafe_code should be forbid".to_owned(),
                message: "unsafe_code = \"deny\" can be overridden per-crate; use \"forbid\""
                    .to_owned(),
                file: Some(cargo_path.display().to_string()),
                line: None,
            });
        }
        _ => {
            // Already covered by R26 lint checks
        }
    }
}


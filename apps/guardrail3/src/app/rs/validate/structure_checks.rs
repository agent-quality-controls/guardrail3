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

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Bug 4: Test file exemptions for R38 ----

    #[test]
    fn file_length_skips_test_files() {
        let long_content = "fn x() {}\n".repeat(600);
        let path = Path::new("/project/tests/my_test.rs");
        let mut results = Vec::new();
        check_file_length(path, &long_content, true, &mut results);
        assert!(
            results.is_empty(),
            "Test files should be exempt from length check"
        );
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn file_length_flags_source_files_over_500() {
        let long_content = "fn x() {}\n".repeat(600);
        let path = Path::new("/project/src/main.rs");
        let mut results = Vec::new();
        check_file_length(path, &long_content, false, &mut results);
        assert!(
            !results.is_empty(),
            "Source files over 500 lines should be flagged"
        );
        assert_eq!(results[0].id, "R38");
        assert_eq!(results[0].severity, Severity::Error);
    }

    #[test]
    fn use_count_skips_test_files() {
        let mut lines: Vec<String> = (0..25).map(|i| format!("use crate::module{i};")).collect();
        lines.push("fn test() {}".to_owned());
        let content = lines.join("\n");
        let path = Path::new("/project/tests/my_test.rs");
        let mut results = Vec::new();
        check_use_count(path, &content, true, &mut results);
        assert!(
            results.is_empty(),
            "Test files should be exempt from use-count check"
        );
    }

    // ---- R40: use count > 20 is Error ----

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn r40_use_count_over_20_is_error() {
        let mut lines: Vec<String> = (0..21).map(|i| format!("use crate::mod{i};")).collect();
        lines.push("fn main() {}".to_owned());
        let content = lines.join("\n");
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_use_count(path, &content, false, &mut results);
        assert!(!results.is_empty(), "Should flag >20 use statements");
        assert_eq!(results[0].id, "R40");
        assert_eq!(results[0].severity, Severity::Error);
    }

    // ---- R41: use count 16-20 is Warn ----

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn r41_use_count_16_is_warn() {
        let mut lines: Vec<String> = (0..16).map(|i| format!("use crate::mod{i};")).collect();
        lines.push("fn main() {}".to_owned());
        let content = lines.join("\n");
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_use_count(path, &content, false, &mut results);
        assert!(!results.is_empty(), "Should warn at 16 use statements");
        assert_eq!(results[0].id, "R41");
        assert_eq!(results[0].severity, Severity::Warn);
    }

    // ---- R42: unsafe usage ----

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn r42_unsafe_block_detected() {
        let content = "fn foo() { unsafe { do_stuff(); } }";
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_unsafe(path, content, &mut results);
        assert!(!results.is_empty(), "Should detect unsafe block");
        assert_eq!(results[0].id, "R42");
        assert_eq!(results[0].severity, Severity::Error);
    }
}

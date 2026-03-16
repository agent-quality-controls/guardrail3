use std::path::Path;

use crate::report::types::{CheckResult, Severity};

use super::ast_helpers;
use super::source_scan::filter_non_comment_lines;

// R43: todo!/unimplemented! (Warn) and unreachable! (Info)
pub fn check_todo_macros(
    path: &Path,
    content: &str,
    is_test_file: bool,
    results: &mut Vec<CheckResult>,
) {
    if let Some(file) = ast_helpers::parse_file(content) {
        // AST path — no false positives from strings, comments, or identifiers
        for (line, name) in ast_helpers::find_forbidden_macros(&file) {
            let message = content
                .lines()
                .nth(line.saturating_sub(1))
                .unwrap_or("")
                .trim();
            match name.as_str() {
                "todo" | "unimplemented" => {
                    results.push(CheckResult {
                        id: "R43".to_owned(),
                        severity: Severity::Warn,
                        title: format!("{name}! macro"),
                        message: message.to_owned(),
                        file: Some(path.display().to_string()),
                        line: Some(line),
                    });
                }
                "unreachable" if !is_test_file => {
                    results.push(CheckResult {
                        id: "R43".to_owned(),
                        severity: Severity::Info,
                        title: "unreachable! macro".to_owned(),
                        message: message.to_owned(),
                        file: Some(path.display().to_string()),
                        line: Some(line),
                    });
                }
                // panic and unreachable-in-tests: not flagged by R43
                _ => {}
            }
        }
    } else {
        // Fallback to grep if parse fails
        check_todo_macros_grep(path, content, is_test_file, results);
    }
}

fn check_todo_macros_grep(
    path: &Path,
    content: &str,
    is_test_file: bool,
    results: &mut Vec<CheckResult>,
) {
    let non_comment_lines = filter_non_comment_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        for macro_name in &["todo!(", "unimplemented!("] {
            if trimmed.contains(macro_name) {
                let line_number = line_num.saturating_add(1);
                results.push(CheckResult {
                    id: "R43".to_owned(),
                    severity: Severity::Warn,
                    title: format!("{} macro", macro_name.trim_end_matches('(')),
                    message: trimmed.to_owned(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }

        // unreachable! is Info — legitimately used in exhaustive matches
        // Skip unreachable! in test files — it's a normal assertion pattern
        if trimmed.contains("unreachable!(") && !is_test_file {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: "R43".to_owned(),
                severity: Severity::Info,
                title: "unreachable! macro".to_owned(),
                message: trimmed.to_owned(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// R44: .unwrap() / .expect()
pub fn check_unwrap_expect(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    if let Some(file) = ast_helpers::parse_file(content) {
        // AST path — no false positives from strings or field names
        for (line, method) in ast_helpers::find_unwrap_expect(&file) {
            let message = content
                .lines()
                .nth(line.saturating_sub(1))
                .unwrap_or("")
                .trim();
            results.push(CheckResult {
                id: "R44".to_owned(),
                severity: Severity::Warn,
                title: format!(".{method}() usage"),
                message: message.to_owned(),
                file: Some(path.display().to_string()),
                line: Some(line),
            });
        }
    } else {
        // Fallback to grep if parse fails
        check_unwrap_expect_grep(path, content, results);
    }
}

fn check_unwrap_expect_grep(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let non_comment_lines = filter_non_comment_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        if trimmed.contains(".unwrap()") {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: "R44".to_owned(),
                severity: Severity::Warn,
                title: ".unwrap() usage".to_owned(),
                message: trimmed.to_owned(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }

        if trimmed.contains(".expect(") {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: "R44".to_owned(),
                severity: Severity::Warn,
                title: ".expect() usage".to_owned(),
                message: trimmed.to_owned(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

/// Check if a file exists at a given root, emitting Info if found, Warn if missing.
pub fn check_file_exists_at_root(
    root: &Path,
    filename: &str,
    check_id: &str,
    found_title: &str,
    missing_title: &str,
    results: &mut Vec<CheckResult>,
) {
    let file_path = root.join(filename);
    if file_path.exists() {
        results.push(CheckResult {
            id: check_id.to_owned(),
            severity: Severity::Info,
            title: found_title.to_owned(),
            message: "Found at project root".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: check_id.to_owned(),
            severity: Severity::Warn,
            title: missing_title.to_owned(),
            message: format!("No {filename} found at project root"),
            file: Some(root.display().to_string()),
            line: None,
        });
    }
}

// R49: CLAUDE.md
pub fn check_claude_md(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    check_file_exists_at_root(
        workspace_root,
        "CLAUDE.md",
        "R49",
        "CLAUDE.md exists",
        "CLAUDE.md missing",
        results,
    );
}

// R58: Direct std::fs usage — belt-and-suspenders check
// Clippy's disallowed_methods doesn't always catch `use std::fs; fs::read_to_string()`
// when the import aliases the module. This source-level scan fills that gap.
//
// Uses syn AST parsing when possible (no false positives on comments/strings).
// Falls back to grep if parsing fails.
pub fn check_direct_fs_usage(
    path: &Path,
    content: &str,
    _is_test: bool,
    results: &mut Vec<CheckResult>,
) {
    // Skip the centralized fs module itself
    if path.ends_with("fs.rs") {
        return;
    }

    if let Some(parsed) = ast_helpers::parse_file(content) {
        check_direct_fs_usage_ast(path, content, &parsed, results);
    } else {
        check_direct_fs_usage_grep(path, content, results);
    }
}

/// AST-based R58: use `find_std_fs_imports` for use-items, grep for inline `std::fs::` calls.
fn check_direct_fs_usage_ast(
    path: &Path,
    content: &str,
    parsed: &syn::File,
    results: &mut Vec<CheckResult>,
) {
    // Use-imports via syn — immune to comments/strings
    for line_num in ast_helpers::find_std_fs_imports(parsed) {
        let trimmed = content
            .lines()
            .nth(line_num.saturating_sub(1))
            .unwrap_or("")
            .trim();
        results.push(CheckResult {
            id: "R58".to_owned(),
            severity: Severity::Error,
            title: "Direct std::fs import".to_owned(),
            message: format!(
                "Use centralized fs module instead of direct std::fs import: {trimmed}"
            ),
            file: Some(path.display().to_string()),
            line: Some(line_num),
        });
    }

    // Inline std::fs:: calls — still grep-based but with cfg(test) skipping
    check_inline_std_fs_calls(path, content, results);
}

/// Grep fallback for unparseable files.
fn check_direct_fs_usage_grep(
    path: &Path,
    content: &str,
    results: &mut Vec<CheckResult>,
) {
    let mut seen_cfg_test = false;
    let mut in_cfg_test_block = false;
    let mut cfg_test_brace_depth: usize = 0;

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("//") || trimmed.starts_with('*') || trimmed.starts_with("/*") {
            continue;
        }

        if trimmed.contains("#[cfg(test)]") && !trimmed.contains('"') {
            seen_cfg_test = true;
        }

        if seen_cfg_test && !in_cfg_test_block && trimmed.contains('{') {
            in_cfg_test_block = true;
            cfg_test_brace_depth = 0;
        }

        if in_cfg_test_block {
            cfg_test_brace_depth += trimmed.matches('{').count();
            cfg_test_brace_depth = cfg_test_brace_depth.saturating_sub(trimmed.matches('}').count());
            if cfg_test_brace_depth == 0 {
                in_cfg_test_block = false;
                seen_cfg_test = false;
            }
            continue;
        }

        if trimmed.starts_with("use std::fs") && !trimmed.contains('"') {
            results.push(CheckResult {
                id: "R58".to_owned(),
                severity: Severity::Error,
                title: "Direct std::fs import".to_owned(),
                message: format!(
                    "Use centralized fs module instead of direct std::fs import: {trimmed}"
                ),
                file: Some(path.display().to_string()),
                line: Some(line_num.saturating_add(1)),
            });
        }

        check_inline_std_fs_line(path, trimmed, line_num, results);
    }
}

/// Inline `std::fs::` call detection shared by both AST and grep paths.
fn check_inline_std_fs_calls(
    path: &Path,
    content: &str,
    results: &mut Vec<CheckResult>,
) {
    let mut seen_cfg_test = false;
    let mut in_cfg_test_block = false;
    let mut cfg_test_brace_depth: usize = 0;

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("//") || trimmed.starts_with('*') || trimmed.starts_with("/*") {
            continue;
        }

        if trimmed.contains("#[cfg(test)]") && !trimmed.contains('"') {
            seen_cfg_test = true;
        }

        if seen_cfg_test && !in_cfg_test_block && trimmed.contains('{') {
            in_cfg_test_block = true;
            cfg_test_brace_depth = 0;
        }

        if in_cfg_test_block {
            cfg_test_brace_depth += trimmed.matches('{').count();
            cfg_test_brace_depth = cfg_test_brace_depth.saturating_sub(trimmed.matches('}').count());
            if cfg_test_brace_depth == 0 {
                in_cfg_test_block = false;
                seen_cfg_test = false;
            }
            continue;
        }

        check_inline_std_fs_line(path, trimmed, line_num, results);
    }
}

fn check_inline_std_fs_line(
    path: &Path,
    trimmed: &str,
    line_num: usize,
    results: &mut Vec<CheckResult>,
) {
    if !path.to_string_lossy().contains("modules/")
        && trimmed.contains("std::fs::")
        && !trimmed.starts_with("//")
        && !trimmed.starts_with('"')
        && !trimmed.contains("\"std::fs::")
        && !trimmed.contains("std::fs::Permissions")
        && !trimmed.contains("std::fs::Metadata")
        && !trimmed.contains("std::fs::DirEntry")
    {
        results.push(CheckResult {
            id: "R58".to_owned(),
            severity: Severity::Error,
            title: "Direct std::fs call".to_owned(),
            message: format!(
                "Use centralized fs module instead of direct std::fs call: {trimmed}"
            ),
            file: Some(path.display().to_string()),
            line: Some(line_num.saturating_add(1)),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Bug 5: R58 direct std::fs detection ----

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn r58_catches_use_std_fs() {
        let content = "use std::fs;\nfn main() {}";
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_direct_fs_usage(path, content, false, &mut results);
        assert!(!results.is_empty(), "Should catch 'use std::fs'");
        assert_eq!(results[0].id, "R58");
    }

    #[test]
    fn r58_allows_fs_module() {
        let content = "use std::fs;\nfn main() {}";
        let path = Path::new("src/fs.rs");
        let mut results = Vec::new();
        check_direct_fs_usage(path, content, false, &mut results);
        assert!(results.is_empty(), "fs.rs should be exempt from R58");
    }

    #[test]
    fn r58_allows_string_literals_in_modules() {
        let content = r#"let ban = "std::fs::read_to_string";"#;
        let path = Path::new("src/modules/clippy.rs");
        let mut results = Vec::new();
        check_direct_fs_usage(path, content, false, &mut results);
        assert!(
            results.is_empty(),
            "String literals in modules/ should be exempt"
        );
    }

    #[test]
    fn r58_allows_type_references() {
        let content = "let p = std::fs::Permissions::from_mode(0o755);";
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_direct_fs_usage(path, content, false, &mut results);
        assert!(
            results.is_empty(),
            "Type references (Permissions) should be exempt"
        );
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn r58_catches_read_to_string() {
        let content = "let s = std::fs::read_to_string(path);";
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_direct_fs_usage(path, content, false, &mut results);
        assert!(
            !results.is_empty(),
            "Direct std::fs::read_to_string should be caught"
        );
        assert_eq!(results[0].id, "R58");
    }

    #[test]
    fn r58_allows_metadata_type() {
        let content = "fn check(m: std::fs::Metadata) {}";
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_direct_fs_usage(path, content, false, &mut results);
        assert!(
            results.is_empty(),
            "std::fs::Metadata type reference should be exempt"
        );
    }

    #[test]
    fn r58_skips_cfg_test_block() {
        let content = "\
fn production_code() {}

#[cfg(test)]
mod tests {
    use std::fs;
    fn helper() {
        let _ = std::fs::read_to_string(\"test.txt\");
    }
}";
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_direct_fs_usage(path, content, false, &mut results);
        assert!(
            results.is_empty(),
            "std::fs usage inside #[cfg(test)] block should not trigger R58, got: {results:?}"
        );
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn r58_still_catches_production_fs_before_cfg_test() {
        let content = "\
use std::fs;

fn production_code() {}

#[cfg(test)]
mod tests {
    use std::fs;
}";
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_direct_fs_usage(path, content, false, &mut results);
        assert_eq!(
            results.len(),
            1,
            "Should catch production std::fs but not the one in #[cfg(test)]"
        );
        assert_eq!(results[0].id, "R58");
        assert_eq!(
            results[0].line,
            Some(1),
            "Should flag line 1 (production code), not line 7 (test code)"
        );
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn r58_catches_production_fs_after_cfg_test() {
        let content = r#"
use std::collections::BTreeMap;

#[cfg(test)]
mod tests {
    use std::fs;
    fn test_foo() { fs::read_to_string("x"); }
}

fn late_production() {
    let _ = std::fs::read_to_string("important.txt");
}
"#;
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_direct_fs_usage(path, content, false, &mut results);
        assert!(
            !results.is_empty(),
            "Should catch production std::fs usage AFTER #[cfg(test)] block, got no results"
        );
        assert_eq!(results[0].id, "R58");
        assert!(
            results[0].line.is_some_and(|l| l > 9), // reason: the hit must be after the test module
            "R58 hit should be on the production line after the test module, got line {:?}",
            results[0].line
        );
    }

    // ---- R43 todo macro tests ----

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn todo_macro_produces_warn() {
        let content = "fn foo() { todo!(); }";
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_todo_macros(path, content, false, &mut results);
        assert!(!results.is_empty());
        assert_eq!(results[0].severity, Severity::Warn);
        assert_eq!(results[0].id, "R43");
    }

    // ---- R44: .unwrap() / .expect() ----

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn r44_unwrap_detected() {
        let content = "fn foo() { let x = some_option().unwrap(); }";
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_unwrap_expect(path, content, &mut results);
        assert!(!results.is_empty(), "Should detect .unwrap()");
        assert_eq!(results[0].id, "R44");
        assert_eq!(results[0].severity, Severity::Warn);
    }
}

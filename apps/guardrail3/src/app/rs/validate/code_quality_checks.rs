use std::path::Path;

use crate::domain::report::{CheckResult, Severity};

use super::ast_helpers;

// R43: todo!/unimplemented! (Warn) and unreachable! (Info)
pub fn check_todo_macros(
    path: &Path,
    content: &str,
    is_test_file: bool,
    results: &mut Vec<CheckResult>,
) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };

    // AST path — no false positives from strings, comments, or identifiers
    for (line, name) in ast_helpers::find_forbidden_macros(&file) {
        let message = content
            .lines()
            .nth(line.saturating_sub(1))
            .unwrap_or("")
            .trim();
        let base_name = name.rsplit("::").next().unwrap_or(&name);
        match base_name {
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
}

// R44: .unwrap() / .expect()
pub fn check_unwrap_expect(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };

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
// Uses syn AST parsing for use-item detection (no false positives on comments/strings).
pub fn check_direct_fs_usage(
    path: &Path,
    content: &str,
    is_test: bool,
    results: &mut Vec<CheckResult>,
) {
    // Skip the centralized fs module itself
    if path.ends_with("fs.rs") {
        return;
    }

    // Skip test files — tests legitimately need std::fs for temp dirs and fixtures
    if is_test {
        return;
    }

    let Some(parsed) = ast_helpers::parse_file(content) else {
        return;
    };

    // Use-imports via syn — immune to comments/strings
    for line_num in ast_helpers::find_std_fs_imports(&parsed) {
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

    // Inline std::fs:: calls via syn expression visitor
    for line_num in ast_helpers::find_inline_std_fs_calls(&parsed) {
        // Skip if already reported by use-import check
        if results
            .iter()
            .any(|r| r.id == "R58" && r.line == Some(line_num))
        {
            continue;
        }
        let trimmed = content
            .lines()
            .nth(line_num.saturating_sub(1))
            .unwrap_or("")
            .trim();
        results.push(CheckResult {
            id: "R58".to_owned(),
            severity: Severity::Error,
            title: "Direct std::fs call".to_owned(),
            message: format!("Use centralized fs module instead of direct std::fs call: {trimmed}"),
            file: Some(path.display().to_string()),
            line: Some(line_num),
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
    fn r58_catches_type_method_call() {
        // std::fs::Permissions::from_mode IS a std::fs call in expression context — should be caught
        let content = "fn foo() { let p = std::fs::Permissions::from_mode(0o755); }";
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_direct_fs_usage(path, content, false, &mut results);
        assert!(
            !results.is_empty(),
            "std::fs::Permissions::from_mode in expression context should be caught"
        );
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn r58_catches_read_to_string() {
        let content = "fn foo() { let s = std::fs::read_to_string(\"x\").unwrap(); }";
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
        // Type in function signature — syn treats this as Type::Path, not Expr::Path
        let content = "fn check(fs: &dyn FileSystem, m: std::fs::Metadata) {}";
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

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn path_qualified_todo_macro_produces_warn() {
        let content = "fn foo() { std::todo!(); }";
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_todo_macros(path, content, false, &mut results);
        assert!(!results.is_empty(), "std::todo!() should be caught by R43");
        assert_eq!(results[0].severity, Severity::Warn);
        assert_eq!(results[0].id, "R43");
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn core_unimplemented_macro_produces_warn() {
        let content = "fn foo() { core::unimplemented!(); }";
        let path = Path::new("src/foo.rs");
        let mut results = Vec::new();
        check_todo_macros(path, content, false, &mut results);
        assert!(
            !results.is_empty(),
            "core::unimplemented!() should be caught by R43"
        );
        assert_eq!(results[0].severity, Severity::Warn);
        assert_eq!(results[0].id, "R43");
    }
}

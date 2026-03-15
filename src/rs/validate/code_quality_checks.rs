use std::path::Path;

use crate::report::types::{CheckResult, Severity};

use super::source_scan::filter_non_comment_lines;

// R43: todo!/unimplemented! (Warn) and unreachable! (Info)
pub fn check_todo_macros(
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

// R49: CLAUDE.md
pub fn check_claude_md(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let claude_path = workspace_root.join("CLAUDE.md");
    if claude_path.exists() {
        results.push(CheckResult {
            id: "R49".to_owned(),
            severity: Severity::Info,
            title: "CLAUDE.md exists".to_owned(),
            message: "Found at project root".to_owned(),
            file: Some(claude_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "R49".to_owned(),
            severity: Severity::Warn,
            title: "CLAUDE.md missing".to_owned(),
            message: "No CLAUDE.md found at project root".to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
        });
    }
}

// R58: Direct std::fs usage — belt-and-suspenders check
// Clippy's disallowed_methods doesn't always catch `use std::fs; fs::read_to_string()`
// when the import aliases the module. This source-level scan fills that gap.
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

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Skip comments
        if trimmed.starts_with("//") || trimmed.starts_with('*') || trimmed.starts_with("/*") {
            continue;
        }

        // Check for `use std::fs` imports (but not in string literals)
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

        // Check for inline std::fs:: calls (not in the fs.rs module itself)
        // Skip files in modules/ directory — they contain string literals with "std::fs::"
        // Skip type references (Permissions, Metadata, DirEntry) — these are not fs operations
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
}

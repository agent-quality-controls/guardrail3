use std::path::Path;

use super::ast_helpers::{self, CommentInfo};
use crate::report::types::{CheckResult, Severity};

/// Determine whether a file path refers to a TSX file.
#[allow(clippy::case_sensitive_file_extension_comparisons)] // reason: only checking .tsx extension
pub(super) fn is_tsx_path(path: &Path) -> bool {
    path.to_string_lossy().ends_with(".tsx")
}

// T23-T26: eslint-disable checks (AST-only)
pub fn check_eslint_disable(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let Some(tree) = ast_helpers::parse_ts_file(content, is_tsx_path(path)) else {
        return;
    };
    let comments = ast_helpers::find_comments(&tree, content);
    check_eslint_disable_from_comments(path, &comments, results);
}

/// Tree-sitter path: only inspect actual comment nodes for eslint-disable patterns.
fn check_eslint_disable_from_comments(
    path: &Path,
    comments: &[CommentInfo],
    results: &mut Vec<CheckResult>,
) {
    for comment in comments {
        let text = comment.text.trim();
        let line_number = comment.line;

        // Block-level eslint-disable (T23/T24)
        if text.contains("eslint-disable")
            && !text.contains("eslint-disable-next-line")
            && !text.contains("eslint-disable-line")
        {
            if text.contains("-- ") {
                results.push(CheckResult {
                    id: "T24".to_owned(),
                    severity: Severity::Info,
                    title: "eslint-disable with reason".to_owned(),
                    message: text.to_owned(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            } else {
                results.push(CheckResult {
                    id: "T23".to_owned(),
                    severity: Severity::Error,
                    title: "eslint-disable without reason".to_owned(),
                    message: format!("eslint-disable missing `-- ` reason: {text}"),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }

        // eslint-disable-next-line (T25/T26)
        if text.contains("eslint-disable-next-line") {
            if text.contains("-- ") {
                results.push(CheckResult {
                    id: "T26".to_owned(),
                    severity: Severity::Info,
                    title: "eslint-disable-next-line with reason".to_owned(),
                    message: text.to_owned(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            } else {
                results.push(CheckResult {
                    id: "T25".to_owned(),
                    severity: Severity::Error,
                    title: "eslint-disable-next-line without reason".to_owned(),
                    message: format!("Missing `-- ` reason: {text}"),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }

        // eslint-disable-line (T25/T26 -- inline suppression)
        if text.contains("eslint-disable-line") && !text.contains("eslint-disable-line-") {
            if text.contains("-- ") {
                results.push(CheckResult {
                    id: "T26".to_owned(),
                    severity: Severity::Info,
                    title: "eslint-disable-line with reason".to_owned(),
                    message: text.to_owned(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            } else {
                results.push(CheckResult {
                    id: "T25".to_owned(),
                    severity: Severity::Error,
                    title: "eslint-disable-line without reason".to_owned(),
                    message: format!("Missing `-- ` reason: {text}"),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }
    }
}

// T27-T29: @ts-ignore / @ts-expect-error (AST-only)
pub fn check_ts_ignore(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let Some(tree) = ast_helpers::parse_ts_file(content, is_tsx_path(path)) else {
        return;
    };
    let comments = ast_helpers::find_comments(&tree, content);
    check_ts_ignore_from_comments(path, &comments, results);
}

/// Tree-sitter path: only inspect actual comment nodes for ts-ignore/ts-expect-error.
fn check_ts_ignore_from_comments(
    path: &Path,
    comments: &[CommentInfo],
    results: &mut Vec<CheckResult>,
) {
    for comment in comments {
        let text = comment.text.trim();
        let line_number = comment.line;

        // T27: @ts-ignore
        if text.contains("@ts-ignore") {
            results.push(CheckResult {
                id: "T27".to_owned(),
                severity: Severity::Error,
                title: "@ts-ignore usage".to_owned(),
                message: format!("Use @ts-expect-error instead: {text}"),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }

        // T28/T29: @ts-expect-error
        if text.contains("@ts-expect-error") {
            if let Some(pos) = text.find("@ts-expect-error") {
                #[allow(clippy::string_slice)] // reason: @ts-expect-error is ASCII, byte offset + 16 is safe
                let after = text.get(pos.saturating_add(16)..).unwrap_or("").trim();
                if after.is_empty() || after == "*/" {
                    results.push(CheckResult {
                        id: "T28".to_owned(),
                        severity: Severity::Warn,
                        title: "@ts-expect-error without explanation".to_owned(),
                        message: text.to_owned(),
                        file: Some(path.display().to_string()),
                        line: Some(line_number),
                    });
                } else {
                    results.push(CheckResult {
                        id: "T29".to_owned(),
                        severity: Severity::Info,
                        title: "@ts-expect-error with explanation".to_owned(),
                        message: text.to_owned(),
                        file: Some(path.display().to_string()),
                        line: Some(line_number),
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn test_eslint_disable_block_no_reason_t23() {
        let content = "const x = 1;\n/* eslint-disable no-console */\nconsole.log(x);\n";
        let path = Path::new("test.ts");
        let mut results = Vec::new();
        check_eslint_disable(path, content, &mut results);
        let errors: Vec<_> = results
            .iter()
            .filter(|r| r.id == "T23" && r.severity == Severity::Error)
            .collect();
        assert!(!errors.is_empty(), "Should produce T23 Error");
        assert_eq!(errors[0].id, "T23");
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn test_eslint_disable_block_with_reason_t24() {
        let content =
            "const x = 1;\n/* eslint-disable no-console -- needed for CLI */\nconsole.log(x);\n";
        let path = Path::new("test.ts");
        let mut results = Vec::new();
        check_eslint_disable(path, content, &mut results);
        let infos: Vec<_> = results
            .iter()
            .filter(|r| r.id == "T24" && r.severity == Severity::Info)
            .collect();
        assert!(!infos.is_empty(), "Should produce T24 Info");
        assert_eq!(infos[0].id, "T24");
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn test_eslint_disable_next_line_no_reason_t25() {
        let content = "// eslint-disable-next-line no-console\nconsole.log('hello');\n";
        let path = Path::new("test.ts");
        let mut results = Vec::new();
        check_eslint_disable(path, content, &mut results);
        let errors: Vec<_> = results
            .iter()
            .filter(|r| r.id == "T25" && r.severity == Severity::Error)
            .collect();
        assert!(!errors.is_empty(), "Should produce T25 Error");
        assert_eq!(errors[0].id, "T25");
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn test_eslint_disable_next_line_with_reason_t26() {
        let content = "// eslint-disable-next-line no-console -- reason\nconsole.log('hello');\n";
        let path = Path::new("test.ts");
        let mut results = Vec::new();
        check_eslint_disable(path, content, &mut results);
        let infos: Vec<_> = results
            .iter()
            .filter(|r| r.id == "T26" && r.severity == Severity::Info)
            .collect();
        assert!(!infos.is_empty(), "Should produce T26 Info");
        assert_eq!(infos[0].id, "T26");
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn test_ts_ignore_t27() {
        let content = "// @ts-ignore\nconst x: number = 'hello';\n";
        let path = Path::new("test.ts");
        let mut results = Vec::new();
        check_ts_ignore(path, content, &mut results);
        let errors: Vec<_> = results
            .iter()
            .filter(|r| r.id == "T27" && r.severity == Severity::Error)
            .collect();
        assert!(!errors.is_empty(), "Should produce T27 Error");
        assert_eq!(errors[0].id, "T27");
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn test_ts_expect_error_no_explanation_t28() {
        let content = "// @ts-expect-error\nconst x: number = 'hello';\n";
        let path = Path::new("test.ts");
        let mut results = Vec::new();
        check_ts_ignore(path, content, &mut results);
        let warns: Vec<_> = results
            .iter()
            .filter(|r| r.id == "T28" && r.severity == Severity::Warn)
            .collect();
        assert!(!warns.is_empty(), "Should produce T28 Warn");
        assert_eq!(warns[0].id, "T28");
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn test_ts_expect_error_with_explanation_t29() {
        let content = "// @ts-expect-error: type mismatch\nconst x: number = 'hello';\n";
        let path = Path::new("test.ts");
        let mut results = Vec::new();
        check_ts_ignore(path, content, &mut results);
        let infos: Vec<_> = results
            .iter()
            .filter(|r| r.id == "T29" && r.severity == Severity::Info)
            .collect();
        assert!(!infos.is_empty(), "Should produce T29 Info");
        assert_eq!(infos[0].id, "T29");
    }
}

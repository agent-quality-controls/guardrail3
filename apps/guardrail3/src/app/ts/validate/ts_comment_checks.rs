use std::path::Path;

use super::ast_helpers::{self, CommentInfo};
use crate::domain::report::{CheckResult, Severity};

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
                #[allow(clippy::string_slice)] // reason: ASCII offset safe
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

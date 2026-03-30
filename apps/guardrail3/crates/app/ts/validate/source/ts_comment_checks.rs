use std::path::Path;

use super::ast_helpers::{self, CommentInfo};
use guardrail3_domain_report::{CheckResult, Severity};

/// Determine whether a file path refers to a TSX file.
pub(super) fn is_tsx_path(path: &Path) -> bool {
    path.extension().is_some_and(|e| e == "tsx")
}

/// Check whether an eslint-disable comment contains a non-empty reason after `--`.
///
/// Accepts: `-- some reason`, `--reason` (no space).
/// Rejects: no `--` at all, `-- ` followed by nothing/whitespace, trailing `--`.
fn has_eslint_reason(text: &str) -> bool {
    // Find the LAST occurrence of "--" to handle rule names containing dashes
    if let Some(pos) = text.rfind("--") {
        let after = text.get(pos.saturating_add(2)..).unwrap_or("");
        // Must have non-whitespace content after the "--"
        !after.trim().is_empty()
    } else {
        false
    }
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
        let text = comment.text().trim();
        let line_number = comment.line();

        check_block_eslint_disable(path, text, line_number, results);
        check_next_line_eslint_disable(path, text, line_number, results);
        check_inline_eslint_disable(path, text, line_number, results);
    }
}

/// T23/T24: Block-level eslint-disable.
fn check_block_eslint_disable(
    path: &Path,
    text: &str,
    line_number: usize,
    results: &mut Vec<CheckResult>,
) {
    if !text.contains("eslint-disable")
        || text.contains("eslint-disable-next-line")
        || text.contains("eslint-disable-line")
    {
        return;
    }
    if has_eslint_reason(text) {
        results.push(
            CheckResult::from_parts(
                "T24".to_owned(),
                Severity::Info,
                "Block eslint-disable with reason".to_owned(),
                format!(
                    "Block-level `eslint-disable` with documented reason: `{text}`. \
                 Block disables suppress ESLint rules for an entire section. Tracked for audit."
                ),
                Some(path.display().to_string()),
                Some(line_number),
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
    "T23".to_owned(),
    Severity::Error,
    "Block eslint-disable without reason".to_owned(),
    format!(
                "Block-level `eslint-disable` missing reason: `{text}`. \
                 Disabling ESLint rules hides potential bugs. Every suppression must document WHY \
                 the rule doesn't apply. Add `-- <reason>` after the rule name, e.g., \
                 `/* eslint-disable no-console -- CLI tool needs console output */`."
            ),
    Some(path.display().to_string()),
    Some(line_number),
    false,
        ));
    }

/// T25/T26: eslint-disable-next-line.
fn check_next_line_eslint_disable(
    path: &Path,
    text: &str,
    line_number: usize,
    results: &mut Vec<CheckResult>,
) {
    if !text.contains("eslint-disable-next-line") {
        return;
    }
    emit_line_suppression_result(path, text, line_number, "Line-level", results);
}

/// T25/T26: eslint-disable-line (inline suppression).
fn check_inline_eslint_disable(
    path: &Path,
    text: &str,
    line_number: usize,
    results: &mut Vec<CheckResult>,
) {
    if !text.contains("eslint-disable-line") || text.contains("eslint-disable-line-") {
        return;
    }
    emit_line_suppression_result(path, text, line_number, "Inline", results);
}

/// Emit a T25 (error, no reason) or T26 (info, with reason) result for line-level suppressions.
fn emit_line_suppression_result(
    path: &Path,
    text: &str,
    line_number: usize,
    kind: &str,
    results: &mut Vec<CheckResult>,
) {
    if has_eslint_reason(text) {
        results.push(
            CheckResult::from_parts(
                "T26".to_owned(),
                Severity::Info,
                format!("{kind} eslint-disable with reason"),
                format!(
                    "{kind} ESLint suppression with documented reason: `{text}`. Tracked for audit."
                ),
                Some(path.display().to_string()),
                Some(line_number),
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
    "T25".to_owned(),
    Severity::Error,
    format!("{kind} eslint-disable without reason"),
    format!(
                "{kind} ESLint suppression missing reason: `{text}`. \
                 Every suppression must explain WHY the rule doesn't apply here. \
                 Add `-- <reason>` after the rule name."
            ),
    Some(path.display().to_string()),
    Some(line_number),
    false,
        ));
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
        let text = comment.text().trim();
        let line_number = comment.line();

        // T27: @ts-ignore
        if text.contains("@ts-ignore") {
            results.push(CheckResult::from_parts(
    "T27".to_owned(),
    Severity::Error,
    "`@ts-ignore` suppresses type checking".to_owned(),
    format!(
                    "`@ts-ignore` found: `{text}`. This suppresses TypeScript type checking on the next line \
                     without explanation, hiding type errors that indicate real bugs. Unlike `@ts-expect-error`, \
                     it stays silent even after the underlying error is fixed, leaving dead suppressions. \
                     Replace with `@ts-expect-error: <reason>` which documents why and fails if the error is resolved."
                ),
    Some(path.display().to_string()),
    Some(line_number),
    false,
            ));
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
                        title: "`@ts-expect-error` without explanation".to_owned(),
                        message: format!(
                            "`@ts-expect-error` without explanation: `{text}`. While better than `@ts-ignore` \
                             (it fails when the error is fixed), it still needs a reason so reviewers understand \
                             why the type error is expected. Add an explanation after the directive: \
                             `// @ts-expect-error: <reason>`."
                        ),
                        file: Some(path.display().to_string()),
                        line: Some(line_number),
                        inventory: false,
                    });
                } else {
                    results.push(CheckResult {
                        id: "T29".to_owned(),
                        severity: Severity::Info,
                        title: "`@ts-expect-error` with explanation".to_owned(),
                        message: format!(
                            "Documented type suppression: `{text}`. This will fail if the underlying \
                             type error is fixed, ensuring the suppression doesn't outlive its need. Tracked for audit."
                        ),
                        file: Some(path.display().to_string()),
                        line: Some(line_number),
                        inventory: false,
                    }.as_inventory());
                }
            }
        }
    },
)

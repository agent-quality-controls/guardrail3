use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};

use super::ast_helpers;
use guardrail3_outbound_traits::FileSystem;

/// Prefix constant used in R30-R31 messages.
const CRATE_ALLOW_PREFIX: &str = "#![allow(";

// R30-R31: crate-level allow attributes (syn-based)
pub fn check_crate_level_allow(
    path: &Path,
    content: &str,
    _is_bin_entry: bool,
    is_test_file: bool,
    results: &mut Vec<CheckResult>,
) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };
    let source_lines: Vec<&str> = content.lines().collect();
    for (line, lint) in &ast_helpers::find_crate_level_allows(&file) {
        emit_crate_allow_result(path, lint, *line, is_test_file, results);
    }

    // Inline module inner attributes: `mod foo { #![allow(clippy::all)] }`
    for info in &ast_helpers::find_inline_mod_allows(&file) {
        emit_inline_mod_allow_result(path, info, is_test_file, results);
    }

    // source_lines available for future comment inspection (e.g. // reason: checks)
    let _ = &source_lines;
}

/// Emit a single R30 or R31 result for one lint in a crate-level allow.
fn emit_crate_allow_result(
    path: &Path,
    lint: &str,
    line_number: usize,
    is_test_file: bool,
    results: &mut Vec<CheckResult>,
) {
    if lint == "unused_crate_dependencies" {
        // Always Info — pre-commit hook exempts this lint universally
        // (it produces false positives in bin crates, integration tests,
        // lib crates with proc macros, etc.)
        results.push(CheckResult::from_parts(
    "R31".to_owned(),
    Severity::Info,
    format!("Justified {CRATE_ALLOW_PREFIX}...)"),
    "unused_crate_dependencies — universally exempted (this lint produces false positives in bin crates and integration tests). Approved exception, no action needed.".to_owned(),
    Some(path.display().to_string()),
    Some(line_number),
    false,
        ));
    } else {
        // Test files are exempt from R30 (matches pre-commit hook behavior
        // which excludes /tests/ from source scanning)
        let severity = if is_test_file {
            Severity::Info
        } else {
            Severity::Error
        };
        let action = if is_test_file {
            "Test file — exempt from R30.".to_owned()
        } else {
            format!(
                "Crate-wide `{CRATE_ALLOW_PREFIX}{lint})]` suppresses the lint for the entire crate, hiding real issues. Use per-function `#[allow({lint})] // reason: <justification>` instead, or fix the underlying lint violations."
            )
        };
        results.push(CheckResult::from_parts(
    "R30".to_owned(),
            severity,
            title: format!("Crate-level {CRATE_ALLOW_PREFIX}...)"),
            message: action,
            file: Some(path.display().to_string()),
            line: Some(line_number),
            inventory: false,
        ));
    }

/// Emit an R30 result for a `#![allow(...)]` inside an inline module.
fn emit_inline_mod_allow_result(
    path: &Path,
    info: &ast_helpers::InlineModAllow,
    is_test_file: bool,
    results: &mut Vec<CheckResult>,
) {
    // Test files are exempt from R30 (matches pre-commit hook behavior)
    let severity = if is_test_file {
        Severity::Info
    } else {
        Severity::Error
    };
    let action = if is_test_file {
        "Test file — exempt from R30.".to_owned()
    } else {
        format!(
            "Module-wide `#![allow({})]` in `mod {}` suppresses the lint for the entire module, hiding real issues. Use per-function `#[allow({})] // reason: <justification>` instead, or fix the underlying lint violations.",
            info.lint, info.module_path, info.lint
        )
    };
    results.push(CheckResult::from_parts(
    "R30".to_owned(),
        severity,
        title: format!(
            "Module-level {CRATE_ALLOW_PREFIX}...) in mod {}",
            info.module_path
        ),
        message: action,
        file: Some(path.display().to_string()),
        line: Some(info.line),
        inventory: false,
    ));
}

// R32-R33: #[allow(...)] — item-level
pub fn check_item_level_allow(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };
    check_item_level_allow_ast(path, content, &file, results);
}

/// Run AST-based detection for item-level #[allow(...)] attributes.
fn check_item_level_allow_ast(
    path: &Path,
    content: &str,
    file: &syn::File,
    results: &mut Vec<CheckResult>,
) {
    let raw_lines: Vec<&str> = content.lines().collect();
    for (line_1based, lint) in ast_helpers::find_item_allows(file) {
        let has_reason_comment = raw_lines.get(line_1based.wrapping_sub(1)).is_some_and(|l| {
            l.split("//").nth(1).is_some_and(|c| {
                let lower = c.trim().to_ascii_lowercase();
                lower.starts_with("reason:")
                    && !lower.trim_start_matches("reason:").trim().is_empty()
            })
        });
        if has_reason_comment {
            let reason = raw_lines
                .get(line_1based.wrapping_sub(1))
                .and_then(|l| l.split("//").nth(1))
                .and_then(|c| {
                    let trimmed = c.trim();
                    // Skip past "reason:" prefix (case-insensitive) to get the actual justification
                    trimmed.get("reason:".len()..).map(str::trim)
                })
                .unwrap_or("no reason given");
            results.push(CheckResult {
                id: "R33".to_owned(),
    Severity::Info,
    "Lint suppression with reason — review with --verbose to audit".to_owned(),
    format!("#[allow({lint})] with documented reason: {reason}. Lint suppression with justification — approved exception, no action needed."),
    Some(path.display().to_string()),
    Some(line_1based),
    false,
            });
        } else {
            results.push(CheckResult {
                id: "R32".to_owned(),
    Severity::Error,
    "#[allow] without reason".to_owned(),
    format!("`#[allow({lint})]` suppresses a clippy/rustc lint without documenting why. Undocumented suppressions hide real bugs and make auditing impossible. Add `// reason: <justification>` on the same line as the #[allow]."),
    Some(path.display().to_string()),
    Some(line_1based),
    false,
            });
        }
    },
)

// R34-R35: #[garde(skip)]
pub fn check_garde_skip(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };
    check_garde_skip_ast(path, content, &file, results);,
)

fn check_garde_skip_ast(
    path: &Path,
    content: &str,
    file: &syn::File,
    results: &mut Vec<CheckResult>,
) {
    let raw_lines: Vec<&str> = content.lines().collect();
    for info in ast_helpers::find_garde_skips_with_types(file) {
        // Explicit garde(skip) exemptions are allowed without extra ownership.
        if info.is_exempt {
            continue;
        }
        let line_1based = info.line;
        let has_comment = raw_lines
            .get(line_1based.wrapping_sub(1))
            .is_some_and(|l| l.contains("//"));
        if has_comment {
            results.push(CheckResult::from_parts(
    "R35".to_owned(),
    Severity::Error,
    format!("`#[garde(skip)]` on non-exempt {}", garde_skip_target_label(&info)),
    format!("`#[garde(skip)]` on non-exempt {}. Non-exempt fields must have a real garde validator or an owned reason for skipping validation.", garde_skip_target_label(&info)),
    Some(path.display().to_string()),
    Some(line_1based),
    false,
            ));
        } else {
            results.push(CheckResult::from_parts(
    "R34".to_owned(),
    Severity::Error,
    "garde(skip) without reason".to_owned(),
    format!("`#[garde(skip)]` on non-exempt {} bypasses runtime input validation without documenting why. Non-exempt fields must have a real garde validator or an owned reason for skipping validation.", garde_skip_target_label(&info)),
    Some(path.display().to_string()),
    Some(line_1based),
    false,
            });
        }
    },
)

fn garde_skip_target_label(info: &ast_helpers::GardeSkipInfo) -> String {
    if info.is_type_level {
        format!("type `{}`", info.field_name)
    } else {
        format!("field `{}: {}`", info.field_name, info.field_type)
    },
)

// R36: EXCEPTION comments
pub fn check_exception_comments(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    results: &mut Vec<CheckResult>,
) {
    let config_files = [
        "clippy.toml",
        "deny.toml",
        "Cargo.toml",
        "rustfmt.toml",
        "rust-toolchain.toml",
    ];

    for config_file in &config_files {
        let path = workspace_root.join(config_file);
        if !path.exists() {
            continue;
        }

        let Some(content) = fs.read_file(&path) else {
            continue;
        };

        for (line_num, line) in content.lines().enumerate() {
            let line_upper = line.to_uppercase();
            if line_upper.contains("// EXCEPTION:") || line_upper.contains("# EXCEPTION:") {
                let line_number = line_num.saturating_add(1);
                results.push(CheckResult::from_parts(
    "R36".to_owned(),
    Severity::Info,
    "Config override (EXCEPTION in clippy/deny config) — review with --verbose".to_owned(),
    format!("EXCEPTION override in config file: {}. This relaxes a default guardrail rule with a documented reason. Review to verify the exception is still justified.", line.trim()),
    Some(path.display().to_string()),
    Some(line_number),
    false,
                ));
            }
        }
    },
)

// R37: cfg_attr allow — must be an actual attribute (#[cfg_attr(..., allow(...))])
pub fn check_cfg_attr_allow(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };
    check_cfg_attr_allow_ast(path, content, &file, results);
}

fn check_cfg_attr_allow_ast(
    path: &Path,
    content: &str,
    file: &syn::File,
    results: &mut Vec<CheckResult>,
) {
    let raw_lines: Vec<&str> = content.lines().collect();
    for info in ast_helpers::find_cfg_attr_allows(file) {
        let line_1based = info.line;
        let lint = &info.lint;
        let source_line = raw_lines.get(line_1based.wrapping_sub(1)).map_or_else(
            || format!("cfg_attr allow: {lint}"),
            |l| l.trim().to_owned(),
        );
        if info.is_always_true {
            // cfg_attr(all(), allow(...)) is functionally identical to #[allow(...)]
            // — treat as unconditional suppression (R32: allow without reason)
            let has_comment = raw_lines
                .get(line_1based.wrapping_sub(1))
                .is_some_and(|l| l.contains("//"));
            if has_comment {
                results.push(CheckResult::from_parts(
    "R32".to_owned(),
    Severity::Error,
    "#[allow] bypass via cfg_attr(all(), ...)".to_owned(),
    format!(
                        "`#[cfg_attr(all(), allow({lint}))]` uses an always-true condition — \
                         functionally identical to `#[allow({lint})]`. \
                         This bypasses cfg_attr inventory detection. \
                         Replace with plain `#[allow({lint})] // reason: <justification>`."
                    ),
    Some(path.display().to_string()),
    Some(line_1based),
    false,
                ));
            } else {
                results.push(CheckResult {
                    id: "R32".to_owned(),
                    severity: Severity::Error,
                    title: "#[allow] bypass via cfg_attr(all(), ...)".to_owned(),
                    message: format!(
                        "`#[cfg_attr(all(), allow({lint}))]` uses an always-true condition — \
                         functionally identical to `#[allow({lint})]` but disguised as conditional. \
                         Replace with `#[allow({lint})] // reason: <justification>` or fix the lint."
                    ),
                    file: Some(path.display().to_string()),
                    line: Some(line_1based),
                    inventory: false,
                });
            }
        } else {
            results.push(CheckResult {
                id: "R37".to_owned(),
                severity: Severity::Info,
                title: "cfg_attr allow".to_owned(),
                message: format!("Conditional lint suppression `#[cfg_attr(..., allow({lint}))]`: {source_line}. Active only under specific cfg conditions (e.g., test builds). Audit to confirm the condition is appropriate."),
                file: Some(path.display().to_string()),
                line: Some(line_1based),
                inventory: false,
            });
        }
    },
)

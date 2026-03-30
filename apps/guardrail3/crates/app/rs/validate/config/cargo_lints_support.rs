use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};

pub(super) struct LintExpectation {
    pub(super) name: &'static str,
    pub(super) expected_level: &'static str,
    pub(super) priority: Option<i64>,
}

pub(super) const EXPECTED_RUST_LINTS: &[LintExpectation] = &[
    LintExpectation {
        name: "warnings",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "unsafe_code",
        expected_level: "forbid",
        priority: None,
    },
    LintExpectation {
        name: "dead_code",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "unused_results",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "unused_crate_dependencies",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "missing_debug_implementations",
        expected_level: "warn",
        priority: None,
    },
];

pub(super) const EXPECTED_CLIPPY_GROUPS: &[LintExpectation] = &[
    LintExpectation {
        name: "all",
        expected_level: "deny",
        priority: Some(-1),
    },
    LintExpectation {
        name: "pedantic",
        expected_level: "deny",
        priority: Some(-1),
    },
    LintExpectation {
        name: "cargo",
        expected_level: "deny",
        priority: Some(-1),
    },
    LintExpectation {
        name: "nursery",
        expected_level: "deny",
        priority: Some(-1),
    },
];

pub(super) const EXPECTED_CLIPPY_DENY: &[&str] = &[
    "unwrap_used",
    "expect_used",
    "panic",
    "unimplemented",
    "todo",
    "dbg_macro",
    "print_stdout",
    "print_stderr",
    "disallowed_methods",
    "disallowed_types",
    "indexing_slicing",
    "string_slice",
    "arithmetic_side_effects",
    "shadow_unrelated",
    "missing_assert_message",
    "partial_pub_fields",
    "str_to_string",
    "implicit_clone",
    "as_conversions",
    "float_cmp",
    "lossy_float_literal",
    "wildcard_enum_match_arm",
    "rest_pat_in_fully_bound_structs",
    "large_stack_arrays",
    "needless_pass_by_value",
    "redundant_else",
    "large_futures",
    "semicolon_if_nothing_returned",
    "redundant_closure_for_method_calls",
    "map_unwrap_or",
    "verbose_file_reads",
];

pub(super) const EXPECTED_CLIPPY_ALLOW: &[&str] = &[
    "missing_docs_in_private_items",
    "module_name_repetitions",
    "must_use_candidate",
    "option_if_let_else",
    "empty_line_after_doc_comments",
    "single_match_else",
    "ref_option_ref",
    "trivially_copy_pass_by_ref",
    "multiple_crate_versions",
];

pub(super) fn get_lint_level(lints: &toml::Value, name: &str) -> Option<String> {
    match lints.get(name) {
        Some(toml::Value::String(s)) => Some(s.clone()),
        Some(toml::Value::Table(table)) => table
            .get("level")
            .and_then(|level| level.as_str())
            .map(std::borrow::ToOwned::to_owned),
        _ => None,
    }
}

pub(super) fn get_lint_priority(lints: &toml::Value, name: &str) -> Option<i64> {
    match lints.get(name) {
        Some(toml::Value::Table(table)) => table.get("priority").and_then(toml::Value::as_integer),
        _ => None,
    }
}

pub(super) struct LintCheck<'a> {
    pub(super) lints: &'a toml::Value,
    pub(super) name: &'a str,
    pub(super) expected_level: &'a str,
    pub(super) expected_priority: Option<i64>,
    pub(super) check_id_missing: &'a str,
    pub(super) check_id_wrong: &'a str,
    pub(super) file_path: &'a Path,
    pub(super) section_hint: Option<&'a str>,
}

pub(super) fn emit_expected_allow_inventory(
    lints: &toml::Value,
    lint_name: &str,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let result = match get_lint_level(lints, lint_name).as_deref() {
        Some("allow") => CheckResult::from_parts(
    "R28".to_owned(),
    Severity::Info,
    format!("Allow deviation: {lint_name}"),
    format!(
                "Clippy lint `{lint_name}` is set to `allow` — intentionally disabled because it produces too many false positives or conflicts with project style. Approved deviation, no action needed."
            ),
    Some(file_path.display().to_string()),
    None,
    false,
        },
        Some(other) => CheckResult {
            id: "R28".to_owned(),
            severity: Severity::Info,
            title: format!("Expected allow: {lint_name}"),
            message: format!(
                "Clippy lint `{lint_name}` = \"{other}\" (expected \"allow\"). This lint is typically allowed but this project enforces it more strictly. Informational, no action needed."
            ),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        },
        None => CheckResult {
            id: "R28".to_owned(),
            severity: Severity::Info,
            title: format!("Expected allow missing: {lint_name}"),
            message: format!(
                "Clippy lint `{lint_name}` is not configured (expected \"allow\"). This lint is typically noisy and allowed, but omission means it defaults to the group level (deny). Consider adding `{lint_name} = \"allow\"` if it produces false positives."
            ),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        },
    };
    results.push(result.as_inventory());,
)

pub(super) fn emit_lint_correct(ctx: &LintCheck<'_>, results: &mut Vec<CheckResult>) {
    if let Some(expected_priority) = ctx.expected_priority {
        let actual_priority = get_lint_priority(ctx.lints, ctx.name);
        if actual_priority == Some(expected_priority) {
            results.push(
                CheckResult::from_parts(
                    ctx.check_id_missing.to_owned(),
                    Severity::Info,
                    format!("{} correct", ctx.name),
                    format!(
                        "{} = {} (priority {expected_priority})",
                        ctx.name, ctx.expected_level
                    ),
                    Some(ctx.file_path.display().to_string()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        } else {
            results.push(CheckResult::from_parts(
    ctx.check_id_wrong.to_owned(),
    Severity::Warn,
    format!("{} priority wrong", ctx.name),
    format!(
                    "Expected priority {expected_priority}, got {}",
                    actual_priority
                        .map_or_else(|| "none".to_owned(), |priority| priority.to_string())
                ),
    Some(ctx.file_path.display().to_string()),
    None,
    false,
            ));
        }
    } else {
        results.push(
            CheckResult::from_parts(
                ctx.check_id_missing.to_owned(),
                Severity::Info,
                format!("{} correct", ctx.name),
                format!("{} = {}", ctx.name, ctx.expected_level),
                Some(ctx.file_path.display().to_string()),
                None,
                false,
            )
            .as_inventory(),
        );
    },
)

pub(super) fn emit_lint_wrong(
    name: &str,
    expected_level: &str,
    actual_level: &str,
    check_id_wrong: &str,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let is_weakened = matches!(
        (expected_level, actual_level),
        ("deny" | "forbid", "warn" | "allow") | ("forbid", "deny")
    );
    results.push(CheckResult::from_parts(
    check_id_wrong.to_owned(),
    if is_weakened {
            Severity::Error
        } else {
            Severity::Warn
        },
    format!("{name} wrong level"),
    format!("Expected \"{expected_level}\", got \"{actual_level}\""),
    Some(file_path.display().to_string()),
    None,
    false,
    ));,
)

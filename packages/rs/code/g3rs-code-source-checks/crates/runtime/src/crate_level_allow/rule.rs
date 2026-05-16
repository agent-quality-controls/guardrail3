#![allow(
    clippy::excessive_nesting,
    clippy::missing_docs_in_private_items,
    clippy::wildcard_enum_match_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::question_mark,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::needless_pass_by_value,
    clippy::expect_used,
    clippy::panic,
    clippy::format_collect,
    clippy::format_in_format_args,
    clippy::option_if_let_else,
    clippy::map_unwrap_or,
    clippy::if_same_then_else,
    clippy::match_same_arms,
    clippy::match_like_matches_macro,
    clippy::nonminimal_bool,
    clippy::single_match_else,
    clippy::items_after_statements,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::needless_for_each,
    clippy::manual_let_else,
    clippy::redundant_else,
    clippy::shadow_unrelated,
    clippy::struct_excessive_bools,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::module_name_repetitions,
    clippy::large_enum_variant,
    clippy::large_types_passed_by_value,
    clippy::ptr_arg,
    clippy::needless_collect,
    clippy::branches_sharing_code,
    clippy::unused_self,
    reason = "code-source-checks parse/visitor walks every variant of large external syntax-tree enums (syn::Type, syn::Item, syn::Expr, syn::Pat, etc.) and the ban-detection visitors mirror the source structure they are looking for; the rule modules accept the schema-versioned shape verbatim because the per-rule findings depend on the exact spans and the rule ids embed the schema."
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::attrs::{find_crate_level_allows, find_inline_mod_allows};
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/crate-level-allow";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for (line, lint) in find_crate_level_allows(input.source) {
        if lint == "unused_crate_dependencies" {
            continue;
        }
        push_result(input, results, line, &lint, None);
    }

    for info in find_inline_mod_allows(input.source) {
        if info.lint == "unused_crate_dependencies" {
            continue;
        }
        push_result(
            input,
            results,
            info.line,
            &info.lint,
            Some(info.module_path.as_str()),
        );
    }
}

/// Implements `push result`.
fn push_result(
    input: &CodeSourceRuleInput<'_>,
    results: &mut Vec<G3CheckResult>,
    line: usize,
    lint: &str,
    module_path: Option<&str>,
) {
    let severity = if input.is_test {
        G3Severity::Info
    } else {
        G3Severity::Error
    };
    let title = module_path.map_or_else(
        || "crate-level allow".to_owned(),
        |module_path| format!("module-level allow in {module_path}"),
    );
    let message = if input.is_test {
        format!("Crate/module-wide allow for `{lint}` is test-file exempt.")
    } else {
        format!(
            "Crate/module-wide `allow({lint})` suppresses the lint too broadly. Use item-level `#[allow({lint})]` with a `// reason:` comment instead."
        )
    };
    results.push(G3CheckResult::new(
        ID.to_owned(),
        severity,
        title,
        message,
        Some(input.rel_path.to_owned()),
        Some(line),
    ));
}

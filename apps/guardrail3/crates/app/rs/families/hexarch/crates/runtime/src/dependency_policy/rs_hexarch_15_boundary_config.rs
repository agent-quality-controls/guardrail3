use guardrail3_domain_report::{CheckResult, Severity};

use crate::inventory::push_success;
use crate::inputs::MemberConfigHexarchInput;

const ID: &str = "RS-HEXARCH-15";

pub fn check(input: &MemberConfigHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let boundary = input.member;
    if let Some(parse_error) = &boundary.parse_error {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "guardrail3.toml parse or validation error blocks hexarch boundary checks"
                .to_owned(),
            format!(
                "Failed to parse or validate `guardrail3.toml`, so guardrail3 cannot fully verify app boundary configuration: {parse_error}"
            ),
            Some("guardrail3.toml".to_owned()),
            None,
            false,
        ));
        return;
    }

    if !boundary.is_app_boundary {
        return;
    }
    if boundary.has_config_entry {
        push_success(
            results,
            ID,
            format!("app boundary `{}` has rust.apps config", boundary.rel_dir),
            format!(
                "App boundary `{}` is covered by an explicit `[rust.apps.*]` configuration entry.",
                boundary.rel_dir
            ),
            Some("guardrail3.toml".to_owned()),
        );
        return;
    }

    let app_name = boundary
        .rel_dir
        .rsplit('/')
        .next()
        .unwrap_or(&boundary.rel_dir);
    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Warn,
    format!("app boundary `{}` missing rust.apps config", boundary.rel_dir),
    format!(
            "Add `[rust.apps.{app_name}]` to `guardrail3.toml` so guardrail3 can enforce app-specific architecture policy."
        ),
    Some("guardrail3.toml".to_owned()),
    None,
    false,
    ));
}

#[cfg(test)]
pub fn check_boundary_config_for_test(
    rel_dir: &str,
    has_config_entry: bool,
    is_app_boundary: bool,
    parse_error: Option<&str>,
) -> Vec<CheckResult> {
    let input = crate::dependency_facts::BoundaryConfigFacts {
        rel_dir: rel_dir.to_owned(),
        has_config_entry,
        is_app_boundary,
        parse_error: parse_error.map(|value| value.to_owned()),
    };
    let mut results = Vec::new();
    check(&MemberConfigHexarchInput::new(&input), &mut results);
    results
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}

#[cfg(test)]
pub(super) fn results_for_test_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_hexarch_15_boundary_config_tests/mod.rs"]
mod rs_hexarch_15_boundary_config_tests;

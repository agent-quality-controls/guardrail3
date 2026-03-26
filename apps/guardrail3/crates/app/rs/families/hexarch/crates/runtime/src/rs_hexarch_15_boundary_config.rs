use guardrail3_domain_report::{CheckResult, Severity};

use crate::MemberConfigHexarchInput;

const ID: &str = "RS-HEXARCH-15";

pub fn check(input: &MemberConfigHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let boundary = input.member;
    if let Some(parse_error) = &boundary.parse_error {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "guardrail3.toml parse error blocks hexarch boundary checks".to_owned(),
            message: format!(
                "Failed to parse `guardrail3.toml`, so guardrail3 cannot verify app boundary configuration: {parse_error}"
            ),
            file: Some("guardrail3.toml".to_owned()),
            line: None,
            inventory: false,
        });
        return;
    }

    if !boundary.is_app_boundary || boundary.has_config_entry {
        return;
    }

    let app_name = boundary
        .rel_dir
        .rsplit('/')
        .next()
        .unwrap_or(&boundary.rel_dir);
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: format!("app boundary `{}` missing rust.apps config", boundary.rel_dir),
        message: format!(
            "Add `[rust.apps.{app_name}]` to `guardrail3.toml` so guardrail3 can enforce app-specific architecture policy."
        ),
        file: Some("guardrail3.toml".to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
pub fn check_boundary_config_for_test(
    rel_dir: &str,
    has_config_entry: bool,
    is_app_boundary: bool,
    parse_error: Option<&str>,
) -> Vec<CheckResult> {
    let input = super::dependency_facts::BoundaryConfigFacts {
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
#[path = "rs_hexarch_15_boundary_config_tests/mod.rs"]
mod rs_hexarch_15_boundary_config_tests;

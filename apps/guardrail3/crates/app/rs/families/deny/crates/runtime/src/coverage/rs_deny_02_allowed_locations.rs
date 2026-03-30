use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ForbiddenDenyConfigInput;

pub fn check(input: &ForbiddenDenyConfigInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.forbidden;
    results.push(CheckResult::from_parts(
        "RS-DENY-02".to_owned(),
        Severity::Error,
        "deny config at forbidden location".to_owned(),
        format!(
            "`{}` ({}) is at `{}` which is not an allowed deny policy root.",
            config.rel_path,
            config.file_kind,
            rel_label(&config.policy_root_rel)
        ),
        Some(config.rel_path.clone()),
        None,
        false,
    ));

    if let Some(parse_error) = &config.parse_error {
        results.push(CheckResult::from_parts(
            "RS-DENY-02".to_owned(),
            Severity::Error,
            "deny config parse error".to_owned(),
            format!("`{}` could not be parsed: {parse_error}", config.rel_path),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }
}

fn rel_label(rel: &str) -> String {
    if rel.is_empty() {
        ".".to_owned()
    } else {
        rel.to_owned()
    }
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use crate::{collected_facts, forbidden_input};
#[cfg(test)]
pub(crate) use ::test_support::{
    build_fixture_deny_toml, copy_fixture, nested_member_shadow_tree, write_file,
};
#[cfg(test)]
#[path = "rs_deny_02_allowed_locations_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_02_allowed_locations_tests;

use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ForbiddenDenyConfigInput;

pub fn check(input: &ForbiddenDenyConfigInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.forbidden;
    results.push(CheckResult {
        id: "RS-DENY-02".to_owned(),
        severity: Severity::Error,
        title: "deny config at forbidden location".to_owned(),
        message: format!(
            "`{}` ({}) is at `{}` which is not an allowed deny policy root.",
            config.rel_path,
            config.file_kind,
            rel_label(&config.policy_root_rel)
        ),
        file: Some(config.rel_path.clone()),
        line: None,
        inventory: false,
    });

    if let Some(parse_error) = &config.parse_error {
        results.push(CheckResult {
            id: "RS-DENY-02".to_owned(),
            severity: Severity::Error,
            title: "deny config parse error".to_owned(),
            message: format!("`{}` could not be parsed: {parse_error}", config.rel_path),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
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
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_forbidden(
    tree: &guardrail3_domain_project_tree::ProjectTree,
    rel_path: &str,
) -> Vec<CheckResult> {
    let facts = crate::collect_facts_for_test(tree);
    let forbidden = facts
        .forbidden_configs
        .iter()
        .find(|config| config.rel_path == rel_path)
        .expect("expected forbidden deny config facts");
    let input = ForbiddenDenyConfigInput::new(forbidden);
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
pub(crate) use crate::{collected_facts, forbidden_input};
#[cfg(test)]
pub(crate) use ::test_support::{build_fixture_deny_toml, copy_fixture, nested_member_shadow_tree, write_file};
#[cfg(test)]
#[path = "rs_deny_02_allowed_locations_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_02_allowed_locations_tests;

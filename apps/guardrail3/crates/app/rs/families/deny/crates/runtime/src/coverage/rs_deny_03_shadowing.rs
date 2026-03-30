use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::{ForbiddenDenyConfigInput, SameRootConflictInput};

pub fn check_forbidden(input: &ForbiddenDenyConfigInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.forbidden;
    if let Some(shadowed_root_rel) = &config.shadowed_root_rel {
        results.push(CheckResult::from_parts(
            "RS-DENY-03".to_owned(),
            Severity::Error,
            "nested deny config shadows parent policy".to_owned(),
            format!(
                "`{}` shadows deny policy rooted at `{}`.",
                config.rel_path,
                rel_label(shadowed_root_rel)
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }
}

pub fn check_same_root_conflict(input: &SameRootConflictInput<'_>, results: &mut Vec<CheckResult>) {
    let conflict = input.conflict;
    results.push(CheckResult::from_parts(
        "RS-DENY-03".to_owned(),
        Severity::Error,
        "multiple deny configs at one policy root".to_owned(),
        format!(
            "`{}` has multiple accepted deny configs: {}.",
            rel_label(&conflict.policy_root_rel),
            conflict.rel_paths.join(", ")
        ),
        conflict.rel_paths.first().cloned(),
        None,
        false,
    ));
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
pub(crate) use crate::{collected_facts, forbidden_input, same_root_conflict_input};
#[cfg(test)]
pub(crate) use ::test_support::{
    build_fixture_deny_toml, copy_fixture, nested_member_shadow_tree, same_root_conflict_tree,
    write_file,
};
#[cfg(test)]
#[path = "rs_deny_03_shadowing_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_03_shadowing_tests;

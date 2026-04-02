use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::SameRootConflictInput;

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
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use crate::{collected_facts, same_root_conflict_input};
#[cfg(test)]
pub(crate) use ::test_support::same_root_conflict_tree;
#[cfg(test)]
#[path = "rs_deny_03_shadowing_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_03_shadowing_tests;

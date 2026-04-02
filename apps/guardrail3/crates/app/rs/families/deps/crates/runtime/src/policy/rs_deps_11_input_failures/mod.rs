use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::InputFailureDepsInput;

const ID: &str = "RS-DEPS-11";

pub fn check(input: &InputFailureDepsInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "dependency policy input failure".to_owned(),
        input.failure.message.clone(),
        Some(input.failure.rel_path.clone()),
        None,
        false,
    ));
}

#[cfg(test)]
fn family_route(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> guardrail3_app_rs_family_mapper::RsDepsRoute {
    let scope = guardrail3_app_rs_structure::collect(tree);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Deps,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::new(tree, &scope, None, &selected, None)
        .map_rs_deps()
}

#[cfg(test)]
pub(super) fn collected_facts(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    installed: &[&str],
) -> crate::facts::DepsFacts {
    crate::facts::collect(
        tree,
        &family_route(tree),
        &test_support::StubToolChecker::new(installed),
    )
}

#[cfg(test)]
pub(super) fn failure_facts(rel_path: &str, message: &str) -> crate::facts::DepsFacts {
    crate::facts::DepsFacts {
        tools: Vec::new(),
        lockfiles: vec![crate::facts::LockfileFacts {
            root_rel_dir: String::new(),
            cargo_lock_rel_path: "Cargo.lock".to_owned(),
            cargo_lock_exists: true,
            cargo_lock_ignored: false,
            gitignore_rel_path: Some(".gitignore".to_owned()),
            profile_name: Some("service".to_owned()),
        }],
        dependency_entries: Vec::new(),
        allowlist_coverage: Vec::new(),
        direct_dependency_caps: Vec::new(),
        input_failures: vec![crate::facts::InputFailureFacts {
            rel_path: rel_path.to_owned(),
            message: message.to_owned(),
        }],
    }
}

#[cfg(test)]
pub(super) fn failure_input<'a>(
    facts: &'a crate::facts::DepsFacts,
    rel_path: &str,
) -> crate::inputs::InputFailureDepsInput<'a> {
    let failure = facts
        .input_failures
        .iter()
        .find(|failure| failure.rel_path == rel_path)
        .expect("expected input failure facts");
    crate::inputs::InputFailureDepsInput::new(failure)
}

#[cfg(test)]
pub(super) fn run_with_facts(
    facts: &crate::facts::DepsFacts,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::run_with_facts(facts)
}

#[cfg(test)]

mod rs_deps_11_input_failures_tests;

use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DirectDependencyCapDepsInput;

const ID: &str = "RS-DEPS-12";
const MAX_UNIQUE_DIRECT_DEPENDENCIES: usize = 25;

pub fn check(input: &DirectDependencyCapDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cap.unique_direct_dependency_count <= MAX_UNIQUE_DIRECT_DEPENDENCIES {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "too many direct dependencies".to_owned(),
        format!(
            "Crate `{}` has {} unique direct dependencies (max {}).",
            input.cap.crate_name,
            input.cap.unique_direct_dependency_count,
            MAX_UNIQUE_DIRECT_DEPENDENCIES
        ),
        Some(input.cap.cargo_rel_path.clone()),
        None,
        false,
    ));
}

#[cfg(test)]
fn family_route(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> guardrail3_app_rs_family_mapper::RsDepsRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Deps,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::new(tree, &scope, None, &selected, None)
        .map_rs_deps()
}

#[cfg(test)]
pub(super) fn collected_facts(
    tree: &guardrail3_domain_project_tree::ProjectTree,
    installed: &[&str],
) -> super::facts::DepsFacts {
    super::facts::collect(
        tree,
        &family_route(tree),
        &test_support::StubToolChecker::new(installed),
    )
}

#[cfg(test)]
pub(super) fn direct_dependency_cap_facts(
    crate_name: &str,
    cargo_rel_path: &str,
    unique_direct_dependency_count: usize,
) -> super::facts::DepsFacts {
    super::facts::DepsFacts {
        tools: Vec::new(),
        lockfiles: Vec::new(),
        dependency_entries: Vec::new(),
        allowlist_coverage: Vec::new(),
        direct_dependency_caps: vec![super::facts::DirectDependencyCapFacts {
            crate_name: crate_name.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            unique_direct_dependency_count,
        }],
        input_failures: Vec::new(),
    }
}

#[cfg(test)]
pub(super) fn run_with_facts(
    facts: &super::facts::DepsFacts,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::run_with_facts(facts)
}

#[cfg(test)]
#[path = "rs_deps_12_direct_dependency_cap_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deps_12_direct_dependency_cap_tests;

use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::AllowlistCoverageDepsInput;

const ID: &str = "RS-DEPS-08";

pub fn check(input: &AllowlistCoverageDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.coverage.profile_name.as_deref() != Some("library") {
        return;
    }

    if input.coverage.has_allowlist {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "library allowlist present".to_owned(),
                format!(
                    "Library crate `{}` has an `allowed_deps` policy.",
                    input.coverage.crate_name
                ),
                Some(input.coverage.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "library allowlist missing".to_owned(),
            format!(
                "Library crate `{}` has no `allowed_deps` policy.",
                input.coverage.crate_name
            ),
            Some(input.coverage.cargo_rel_path.clone()),
            None,
            false,
        ));
    }
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
pub(super) fn coverage_facts(
    profile_name: Option<&str>,
    has_allowlist: bool,
) -> super::facts::DepsFacts {
    super::facts::DepsFacts {
        tools: Vec::new(),
        lockfiles: vec![super::facts::LockfileFacts {
            root_rel_dir: String::new(),
            cargo_lock_rel_path: "Cargo.lock".to_owned(),
            cargo_lock_exists: true,
            cargo_lock_ignored: false,
            gitignore_rel_path: Some(".gitignore".to_owned()),
            profile_name: Some("service".to_owned()),
        }],
        dependency_entries: Vec::new(),
        allowlist_coverage: vec![super::facts::AllowlistCoverageFacts {
            crate_name: "core".to_owned(),
            cargo_rel_path: "packages/core/Cargo.toml".to_owned(),
            profile_name: profile_name.map(str::to_owned),
            has_allowlist,
        }],
        direct_dependency_caps: Vec::new(),
        input_failures: Vec::new(),
    }
}

#[cfg(test)]
pub(super) fn coverage_input<'a>(
    facts: &'a super::facts::DepsFacts,
    cargo_rel_path: &str,
) -> super::inputs::AllowlistCoverageDepsInput<'a> {
    let coverage = facts
        .allowlist_coverage
        .iter()
        .find(|coverage| coverage.cargo_rel_path == cargo_rel_path)
        .expect("expected allowlist coverage facts");
    super::inputs::AllowlistCoverageDepsInput::new(coverage)
}

#[cfg(test)]
pub(super) fn run_with_facts(
    facts: &super::facts::DepsFacts,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::run_with_facts(facts)
}

#[cfg(test)]
#[path = "rs_deps_08_library_allowlist_present_tests/mod.rs"]
mod rs_deps_08_library_allowlist_present_tests;

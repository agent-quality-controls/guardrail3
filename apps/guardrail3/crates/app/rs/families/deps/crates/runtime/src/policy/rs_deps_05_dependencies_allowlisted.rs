use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::DependencySectionKind;
use super::inputs::DependencyEntryDepsInput;

const ID: &str = "RS-DEPS-05";

pub fn check(input: &DependencyEntryDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.entry.section_kind != DependencySectionKind::Dependencies {
        return;
    }
    if !input.entry.allowlist_present {
        return;
    }

    if input.entry.allowlisted {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "dependency allowlisted".to_owned(),
                format!(
                    "Dependency `{}` in `{}` is allowlisted for crate `{}`.",
                    input.entry.dep_package_name, input.entry.table_label, input.entry.crate_name
                ),
                Some(input.entry.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "unauthorized dependency".to_owned(),
        format!(
            "Dependency `{}` in `{}` is not allowlisted for crate `{}`.",
            input.entry.dep_package_name, input.entry.table_label, input.entry.crate_name
        ),
        Some(input.entry.cargo_rel_path.clone()),
        None,
        false,
    ));
}

#[cfg(test)]
fn family_route(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> guardrail3_app_rs_family_mapper::RsDepsRoute {
    family_route_with_validation_scope(tree, None)
}

#[cfg(test)]
fn family_route_with_validation_scope(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    validation_scope: Option<&str>,
) -> guardrail3_app_rs_family_mapper::RsDepsRoute {
    let scope = guardrail3_app_rs_structure::collect(tree);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Deps,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::new(tree, &scope, None, &selected, None)
        .with_validation_scope(validation_scope)
        .map_rs_deps()
}

#[cfg(test)]
pub(super) fn collected_facts(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    installed: &[&str],
) -> super::facts::DepsFacts {
    super::facts::collect(
        tree,
        &family_route(tree),
        &test_support::StubToolChecker::new(installed),
    )
}

#[cfg(test)]
pub(super) fn collected_facts_with_validation_scope(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    installed: &[&str],
    validation_scope: Option<&str>,
) -> super::facts::DepsFacts {
    super::facts::collect(
        tree,
        &family_route_with_validation_scope(tree, validation_scope),
        &test_support::StubToolChecker::new(installed),
    )
}

#[cfg(test)]
pub(super) fn dependency_facts(
    allowlist_present: bool,
    allowlisted: bool,
    dep_package_name: &str,
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
        dependency_entries: vec![super::facts::DependencyEntryFacts {
            crate_name: "api".to_owned(),
            cargo_rel_path: "crates/api/Cargo.toml".to_owned(),
            section_kind: super::facts::DependencySectionKind::Dependencies,
            table_label: "[dependencies]".to_owned(),
            dep_package_name: dep_package_name.to_owned(),
            allowlist_present,
            allowlisted,
        }],
        allowlist_coverage: Vec::new(),
        direct_dependency_caps: Vec::new(),
        input_failures: Vec::new(),
    }
}

#[cfg(test)]
pub(super) fn dependency_input<'a>(
    facts: &'a super::facts::DepsFacts,
    cargo_rel_path: &str,
    dep_package_name: &str,
) -> super::inputs::DependencyEntryDepsInput<'a> {
    let entry = facts
        .dependency_entries
        .iter()
        .find(|entry| {
            entry.cargo_rel_path == cargo_rel_path
                && entry.section_kind == super::facts::DependencySectionKind::Dependencies
                && entry.dep_package_name == dep_package_name
        })
        .expect("expected dependency entry facts");
    super::inputs::DependencyEntryDepsInput::new(entry)
}

#[cfg(test)]
pub(super) fn run_with_facts(
    facts: &super::facts::DepsFacts,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::run_with_facts(facts)
}

#[cfg(test)]
#[path = "rs_deps_05_dependencies_allowlisted_tests/mod.rs"]
mod rs_deps_05_dependencies_allowlisted_tests;

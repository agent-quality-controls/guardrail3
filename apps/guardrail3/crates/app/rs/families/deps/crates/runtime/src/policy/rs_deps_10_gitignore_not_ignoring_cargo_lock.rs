use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::LockfileDepsInput;

const ID: &str = "RS-DEPS-10";

pub fn check(input: &LockfileDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.lockfile.cargo_lock_ignored {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "Cargo.lock ignored in gitignore".to_owned(),
            format!(
                "`{}` ignores `{}` for Rust root `{}`.",
                input
                    .lockfile
                    .gitignore_rel_path
                    .as_deref()
                    .unwrap_or(".gitignore"),
                input.lockfile.cargo_lock_rel_path,
                rel_label(&input.lockfile.root_rel_dir)
            ),
            input.lockfile.gitignore_rel_path.clone(),
            None,
            false,
        ));
    } else {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "Cargo.lock tracked by git".to_owned(),
                format!(
                    "No relevant `.gitignore` masks `{}` for Rust root `{}`.",
                    input.lockfile.cargo_lock_rel_path,
                    rel_label(&input.lockfile.root_rel_dir)
                ),
                Some(input.lockfile.cargo_lock_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
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
pub(super) fn lockfile_facts(
    cargo_lock_exists: bool,
    cargo_lock_ignored: bool,
    root_profile_name: Option<&str>,
) -> super::facts::DepsFacts {
    super::facts::DepsFacts {
        tools: Vec::new(),
        lockfiles: vec![super::facts::LockfileFacts {
            root_rel_dir: String::new(),
            cargo_lock_rel_path: "Cargo.lock".to_owned(),
            cargo_lock_exists,
            cargo_lock_ignored,
            gitignore_rel_path: Some(".gitignore".to_owned()),
            profile_name: root_profile_name.map(str::to_owned),
        }],
        dependency_entries: Vec::new(),
        allowlist_coverage: Vec::new(),
        direct_dependency_caps: Vec::new(),
        input_failures: Vec::new(),
    }
}

#[cfg(test)]
pub(super) fn lockfile_input<'a>(
    facts: &'a super::facts::DepsFacts,
) -> super::inputs::LockfileDepsInput<'a> {
    super::inputs::LockfileDepsInput::new(facts.lockfiles.first().expect("expected lockfile facts"))
}

#[cfg(test)]
pub(super) fn run_with_facts(
    facts: &super::facts::DepsFacts,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::run_with_facts(facts)
}

#[cfg(test)]
#[path = "rs_deps_10_gitignore_not_ignoring_cargo_lock_tests/mod.rs"]
mod rs_deps_10_gitignore_not_ignoring_cargo_lock_tests;

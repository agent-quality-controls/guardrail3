use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-RELEASE-11";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    if krate.description_present || krate.license_present || krate.repository_present {
        return;
    }
    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Warn,
    format!("{} may be accidentally publishable", krate.name),
    format!(
            "Crate `{}` is publishable but missing description, license, and repository metadata. If it is internal, set `publish = false`.",
            krate.name
        ),
    Some(krate.cargo_rel_path.clone()),
    None,
    false,
    ));
}

#[cfg(test)]
pub(super) fn run_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree(tree, tc, thorough)
}
#[cfg(test)]
pub(super) fn crate_facts(name: &str) -> crate::facts::PublishableCrateFacts {
    crate::test_fixtures::crate_facts(name)
}

#[cfg(test)]
pub(super) fn crate_input(
    krate: &crate::facts::PublishableCrateFacts,
) -> crate::inputs::PublishableCrateReleaseInput<'_> {
    crate::test_fixtures::crate_input(krate)
}
#[cfg(test)]
pub(super) use test_support::{StubToolChecker, dir_entry, project_tree, temp_root};

#[cfg(test)]
#[path = "rs_release_11_accidentally_publishable_internal_crates_tests/mod.rs"]
mod rs_release_11_accidentally_publishable_internal_crates_tests;

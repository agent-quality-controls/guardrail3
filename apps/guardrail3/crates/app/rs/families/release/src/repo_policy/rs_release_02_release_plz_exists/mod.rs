use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-02";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let repo = input.repo;
    if repo.release_plz_exists {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "release-plz.toml exists".to_owned(),
                "Repo root includes `release-plz.toml`.".to_owned(),
                Some(repo.release_plz_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Warn,
        "release-plz.toml missing".to_owned(),
        "Repo root is missing `release-plz.toml`.".to_owned(),
        Some(repo.release_plz_rel_path.clone()),
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
pub(super) fn run_tree_with_validation_scope(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
    validation_scope: &str,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree_with_validation_scope(tree, tc, thorough, validation_scope)
}
#[cfg(test)]
pub(super) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::test_fixtures::repo_facts()
}

#[cfg(test)]
pub(super) fn repo_input(
    repo: &crate::facts::RepoReleaseFacts,
) -> crate::inputs::RepoReleaseInput<'_> {
    crate::test_fixtures::repo_input(repo)
}
#[cfg(test)]
pub(super) use test_support::{StubToolChecker, dir_entry, project_tree, temp_root};

#[cfg(test)]

mod rs_release_02_release_plz_exists_tests;

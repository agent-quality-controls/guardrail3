use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-01";
const ALLOWED_LICENSE_PATHS: &[&str] = &["LICENSE", "LICENSE-MIT", "LICENSE-APACHE", "LICENSE.md"];

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    match &input.repo.license_rel_path {
        Some(rel_path) if ALLOWED_LICENSE_PATHS.contains(&rel_path.as_str()) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "LICENSE file exists".to_owned(),
                message: format!("Repo root includes `{rel_path}`."),
                file: Some(rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        _ => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "LICENSE file missing".to_owned(),
            message: "Repo root is missing LICENSE material (`LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, or `LICENSE.md`).".to_owned(),
            file: Some(input.repo.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
pub(super) fn run_tree(
    tree: &guardrail3_domain_project_tree::ProjectTree,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree(tree, tc, thorough)
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
#[path = "rs_release_01_license_file_tests/mod.rs"]
mod rs_release_01_license_file_tests;

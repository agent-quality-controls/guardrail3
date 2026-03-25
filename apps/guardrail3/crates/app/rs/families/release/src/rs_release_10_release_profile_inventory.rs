use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-10";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    if input.repo.release_profile_settings.is_empty() {
        return;
    }
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "Release profile inventory".to_owned(),
            message: format!(
                "Root `[profile.release]` settings: {}.",
                input.repo.release_profile_settings.join(", ")
            ),
            file: Some(input.repo.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

#[cfg(test)]
#[path = "rs_release_10_release_profile_inventory_tests/mod.rs"]
mod tests;

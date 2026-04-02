use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-10";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    if input.repo.release_profile_settings.is_empty() {
        return;
    }
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "Release profile inventory".to_owned(),
            format!(
                "Root `[profile.release]` settings: {}.",
                input.repo.release_profile_settings.join(", ")
            ),
            Some(input.repo.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}


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
        "Repo root is missing `release-plz.toml`. Create `release-plz.toml` at the repo root.".to_owned(),
        Some(repo.release_plz_rel_path.clone()),
        None,
        false,
    ));
}


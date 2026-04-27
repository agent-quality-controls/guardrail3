use g3rs_release_types::G3RsReleaseFileTreeRepo;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-release/release-plz-exists";

pub(crate) fn check(repo: &G3RsReleaseFileTreeRepo, results: &mut Vec<G3CheckResult>) {
    if repo.publishable_count == 0 {
        return;
    }

    if repo.release_plz_exists {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "release-plz.toml exists".to_owned(),
                "Repo root includes `release-plz.toml`.".to_owned(),
                Some(repo.release_plz_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "release-plz.toml missing".to_owned(),
            "Repo root is missing `release-plz.toml`. Create `release-plz.toml` at the repo root."
                .to_owned(),
            Some(repo.release_plz_rel_path.clone()),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "release_plz_exists_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod release_plz_exists_tests;

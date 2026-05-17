use g3rs_release_types::G3RsReleaseFileTreeRepo;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Rule identifier.
const ID: &str = "g3rs-release/cliff-exists";

/// Run this rule and append findings to `results`.
pub(crate) fn check(repo: &G3RsReleaseFileTreeRepo, results: &mut Vec<G3CheckResult>) {
    if repo.publishable_count == 0 {
        return;
    }

    if repo.cliff_exists {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "cliff.toml exists".to_owned(),
                "Repo root includes `cliff.toml`.".to_owned(),
                Some(repo.cliff_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "cliff.toml missing".to_owned(),
            "Repo root is missing `cliff.toml`. Create `cliff.toml` at the repo root.".to_owned(),
            Some(repo.cliff_rel_path.clone()),
            None,
        ));
    }
}

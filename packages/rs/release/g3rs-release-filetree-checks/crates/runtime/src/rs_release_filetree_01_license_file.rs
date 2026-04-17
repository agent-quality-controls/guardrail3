use g3rs_release_types::G3RsReleaseFileTreeRepo;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-RELEASE-FILETREE-01";
const ALLOWED_LICENSE_PATHS: &[&str] = &["LICENSE", "LICENSE-MIT", "LICENSE-APACHE", "LICENSE.md"];

pub(crate) fn check(repo: &G3RsReleaseFileTreeRepo, results: &mut Vec<G3CheckResult>) {
    if repo.publishable_count == 0 {
        return;
    }

    match &repo.license_rel_path {
        Some(rel_path) if ALLOWED_LICENSE_PATHS.contains(&rel_path.as_str()) => {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "LICENSE file exists".to_owned(),
                    format!("Repo root includes `{rel_path}`."),
                    Some(rel_path.clone()),
                    None,
                )
                .into_inventory(),
            );
        }
        _ => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "LICENSE file missing".to_owned(),
                "Repo root is missing LICENSE material (`LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, or `LICENSE.md`). Create a LICENSE file at the repo root.".to_owned(),
                Some(repo.cargo_rel_path.clone()),
                None,
            ));
        }
    }
}

#[cfg(test)]
#[path = "rs_release_filetree_01_license_file_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rs_release_filetree_01_license_file_tests;

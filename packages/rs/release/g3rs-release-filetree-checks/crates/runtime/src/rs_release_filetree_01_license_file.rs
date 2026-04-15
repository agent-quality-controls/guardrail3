use g3rs_release_filetree_checks_types::G3RsReleaseFileTreeRepo;
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
mod tests {
    use super::check;
    use crate::test_support::repo_input;

    #[test]
    fn inventories_when_license_exists() {
        let input = repo_input();
        let mut results = Vec::new();
        check(input.repo.as_ref().unwrap(), &mut results);
        assert_eq!(results[0].id(), "RS-RELEASE-FILETREE-01");
        assert_eq!(results[0].title(), "LICENSE file exists");
        assert!(results[0].inventory());
    }

    #[test]
    fn errors_when_license_is_missing() {
        let mut input = repo_input();
        input.repo.as_mut().unwrap().license_rel_path = None;
        let mut results = Vec::new();

        check(input.repo.as_ref().unwrap(), &mut results);

        assert_eq!(results[0].id(), "RS-RELEASE-FILETREE-01");
        assert_eq!(results[0].title(), "LICENSE file missing");
        assert!(!results[0].inventory());
    }

    #[test]
    fn skips_when_workspace_has_no_publishable_crates() {
        let mut input = repo_input();
        let repo = input.repo.as_mut().unwrap();
        repo.publishable_count = 0;
        repo.license_rel_path = None;
        let mut results = Vec::new();

        check(input.repo.as_ref().unwrap(), &mut results);

        assert!(results.is_empty());
    }
}

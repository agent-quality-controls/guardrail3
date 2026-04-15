use g3rs_release_filetree_checks_types::G3RsReleaseFileTreeRepo;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-RELEASE-FILETREE-03";

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

#[cfg(test)]
mod tests {
    use super::check;
    use crate::test_support::repo_input;

    #[test]
    fn warns_when_cliff_is_missing() {
        let mut input = repo_input();
        input.repo.as_mut().unwrap().cliff_exists = false;
        let mut results = Vec::new();

        check(input.repo.as_ref().unwrap(), &mut results);

        assert_eq!(results[0].id(), "RS-RELEASE-FILETREE-03");
        assert_eq!(results[0].title(), "cliff.toml missing");
        assert!(!results[0].inventory());
    }

    #[test]
    fn skips_when_workspace_has_no_publishable_crates() {
        let mut input = repo_input();
        let repo = input.repo.as_mut().unwrap();
        repo.publishable_count = 0;
        repo.cliff_exists = false;
        let mut results = Vec::new();

        check(input.repo.as_ref().unwrap(), &mut results);

        assert!(results.is_empty());
    }
}

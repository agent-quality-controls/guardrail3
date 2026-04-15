use g3rs_release_filetree_checks_types::G3RsReleaseFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsReleaseFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for failure in &input.input_failures {
        crate::rs_release_filetree_05_input_failures::check(failure, &mut results);
    }

    if let Some(repo) = &input.repo {
        crate::rs_release_filetree_01_license_file::check(repo, &mut results);
        crate::rs_release_filetree_02_release_plz_exists::check(repo, &mut results);
        crate::rs_release_filetree_03_cliff_exists::check(repo, &mut results);
    }

    for readme in &input.readmes {
        crate::rs_release_filetree_04_readme_exists::check(readme, &mut results);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::test_support::{failure, readme, repo_input};

    #[test]
    fn aggregates_repo_readme_and_input_failures() {
        let mut input = repo_input();
        input.readmes.push(readme("demo"));
        input.input_failures.push(failure(
            "crates/demo/README.md",
            "Failed to read README for release checks.",
        ));

        let results = check(&input);

        assert_eq!(results.len(), 5);
        assert!(results.iter().any(|result| result.id() == "RS-RELEASE-FILETREE-01"));
        assert!(results.iter().any(|result| result.id() == "RS-RELEASE-FILETREE-02"));
        assert!(results.iter().any(|result| result.id() == "RS-RELEASE-FILETREE-03"));
        assert!(results.iter().any(|result| result.id() == "RS-RELEASE-FILETREE-04"));
        assert!(results.iter().any(|result| result.id() == "RS-RELEASE-FILETREE-05"));
    }

    #[test]
    fn skips_workspace_release_files_when_nothing_publishes() {
        let mut input = repo_input();
        let repo = input.repo.as_mut().unwrap();
        repo.publishable_count = 0;
        repo.license_rel_path = None;
        repo.release_plz_exists = false;
        repo.cliff_exists = false;

        let results = check(&input);

        assert!(results.is_empty(), "{results:#?}");
    }
}

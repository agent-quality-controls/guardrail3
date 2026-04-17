use g3rs_release_types::G3RsReleaseFileTreeChecksInput;
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
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;

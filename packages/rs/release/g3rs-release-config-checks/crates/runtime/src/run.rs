use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run all release config checks and return the collected results.
pub fn check(input: &G3RsReleaseConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for failure in &input.input_failures {
        crate::rs_release_config_25_input_failures::check(failure, &mut results);
    }
    for krate in &input.crates {
        crate::rs_release_config_00_publish_must_be_explicit::check(krate, &mut results);
        crate::rs_release_config_01_description_present::check(krate, &mut results);
        crate::rs_release_config_02_license_present::check(krate, &mut results);
        crate::rs_release_config_03_repository_present::check(krate, &mut results);
        crate::rs_release_config_04_keywords_present::check(krate, &mut results);
        crate::rs_release_config_05_categories_present::check(krate, &mut results);
        crate::rs_release_config_06_valid_semver::check(krate, &mut results);
        crate::rs_release_config_07_docs_rs_metadata::check(krate, &mut results);
        crate::rs_release_config_08_binstall_metadata::check(krate, &mut results);
        crate::rs_release_config_09_accidentally_publishable::check(krate, &mut results);
        crate::rs_release_config_18_publish_dry_run::check(krate, &mut results);
        crate::rs_release_config_22_include_exclude_inventory::check(krate, &mut results);
        crate::rs_release_config_23_binary_release_workflow::check(krate, &mut results);
        crate::rs_release_config_24_linux_release_target::check(krate, &mut results);
    }
    if let Some(repo) = &input.repo {
        crate::rs_release_config_10_release_plz_baseline::check(repo, &mut results);
        crate::rs_release_config_11_cliff_baseline::check(repo, &mut results);
        crate::rs_release_config_15_semver_checks_installed::check(repo, &mut results);
        crate::rs_release_config_16_publish_status_inventory::check(repo, &mut results);
        crate::rs_release_config_17_release_profile_inventory::check(repo, &mut results);
        crate::rs_release_config_21_crate_inventory::check(repo, &mut results);
    }
    for edge in &input.edges {
        crate::rs_release_config_19_no_path_deps_to_unpublishable::check(edge, &mut results);
        crate::rs_release_config_20_interdependent_version_consistency::check(edge, &mut results);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::check;

    #[test]
    fn skips_repo_level_release_setup_when_nothing_publishes() {
        let mut input = crate::test_support::config_input_for_repo(
            Some(
                r#"
[[package]]
name = "some-crate"
"#,
            ),
            Some("# empty cliff.toml\n"),
        );
        let repo = input.repo.as_mut().expect("repo should exist");
        repo.publishable_count = 0;
        repo.non_publishable_count = 2;
        repo.semver_checks_installed = false;
        repo.publish_setting = Some("false".to_owned());
        repo.release_profile_settings = vec!["lto = true".to_owned()];

        let results = check(&input);

        assert!(results.is_empty(), "{results:#?}");
    }
}

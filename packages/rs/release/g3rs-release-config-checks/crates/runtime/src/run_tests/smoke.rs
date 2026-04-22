use g3rs_release_config_checks_assertions::run as assertions;

#[test]
fn skips_repo_level_release_setup_when_nothing_publishes() {
    let mut input = super::super::repo_input(
        Some(
            r#"
[[package]]
name = "some-crate"
"#,
        ),
        Some("# empty cliff.toml\n"),
    );
    let repo = input
        .repo_checks
        .first_mut()
        .expect("repo check should exist");
    repo.publishable_count = 0;
    repo.non_publishable_count = 2;
    repo.semver_checks_installed = false;
    repo.publish_setting = Some("false".to_owned());
    repo.release_profile_settings = vec!["lto = true".to_owned()];

    let results = super::super::check(&input);

    assertions::assert_no_findings(&results);
}

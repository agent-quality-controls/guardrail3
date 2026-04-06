use g3rs_release_config_checks_assertions::rs_release_config_10_release_plz_baseline as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_changelog_config_wrong() {
    let toml = "\
[workspace]
changelog_config = \"wrong.toml\"
git_release_enable = true
release_always = false
";
    let results = run_check(toml);

    let findings = assertions::findings(&results);
    assert!(
        findings
            .iter()
            .any(|f| f.title == "release-plz: changelog_config should be \"cliff.toml\""),
        "expected wrong changelog_config warning, got: {findings:?}",
    );
}

use g3rs_release_config_checks_assertions::release_plz_baseline::rule as assertions;

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

    assertions::assert_contains(
        &results,
        &[assertions::warn(
            "release-plz: changelog_config should be \"cliff.toml\"",
            "Set changelog_config = \"cliff.toml\" in [workspace].",
            "release-plz.toml",
            false,
        )],
    );
}

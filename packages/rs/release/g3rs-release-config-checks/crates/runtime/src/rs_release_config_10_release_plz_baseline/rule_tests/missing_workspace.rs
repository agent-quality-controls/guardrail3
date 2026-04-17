use g3rs_release_config_checks_assertions::rs_release_config_10_release_plz_baseline::rule as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_workspace_section_missing() {
    let toml = "\
[[package]]
name = \"some-crate\"
";
    let results = run_check(toml);

    assertions::assert_contains(
        &results,
        &[assertions::warn(
            "release-plz: missing [workspace] section",
            "release-plz.toml should have a [workspace] section.",
            "release-plz.toml",
            false,
        )],
    );
}

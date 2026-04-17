use g3rs_release_config_checks_assertions::rs_release_config_11_cliff_baseline::rule as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_git_section_missing() {
    let toml = "\
# empty cliff.toml
";
    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "cliff: missing [git] section",
            "cliff.toml should have a [git] section.",
            "cliff.toml",
            false,
        )],
    );
}

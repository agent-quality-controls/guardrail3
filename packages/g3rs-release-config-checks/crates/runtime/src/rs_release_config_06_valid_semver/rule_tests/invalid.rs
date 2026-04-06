use g3rs_release_config_checks_assertions::rs_release_config_06_valid_semver as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_version_not_semver() {
    let toml = "\
[package]
name = \"bad-ver\"
version = \"not-semver\"
edition = \"2024\"
description = \"A crate with bad version\"
";
    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "bad-ver: invalid version",
            "Version \"not-semver\" does not have at least a major.minor format.",
            "Cargo.toml",
            false,
        )],
    );
}

use g3rs_release_config_checks_assertions::rs_release_config_06_valid_semver as assertions;

use super::helpers::run_check;

#[test]
fn info_when_version_inherited_from_workspace() {
    let toml = "\
[package]
name = \"ws-ver\"
version.workspace = true
edition = \"2024\"
description = \"A crate with workspace version\"
";
    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "ws-ver: valid semver",
            "",
            "Cargo.toml",
            true,
        )],
    );
}

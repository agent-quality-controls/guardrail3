use g3rs_release_config_checks_assertions::rs_release_config_06_valid_semver as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_workspace_inherited_version_has_no_workspace_value() {
    let toml = "\
[package]
name = \"member-crate\"
version = { workspace = true }
edition = \"2024\"
";

    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "member-crate: invalid version",
            "Publishable crates must have a version field in [package].",
            "Cargo.toml",
            false,
        )],
    );
}

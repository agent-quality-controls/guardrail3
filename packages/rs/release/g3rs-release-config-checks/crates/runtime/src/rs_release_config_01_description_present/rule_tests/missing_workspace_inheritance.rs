use g3rs_release_config_checks_assertions::rs_release_config_01_description_present::rule as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_workspace_inherited_description_has_no_workspace_value() {
    let toml = "\
[package]
name = \"member-crate\"
version = \"0.1.0\"
edition = \"2024\"
description = { workspace = true }
";

    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "member-crate: missing description",
            "Publishable crates must have a description field in [package].",
            "Cargo.toml",
            false,
        )],
    );
}

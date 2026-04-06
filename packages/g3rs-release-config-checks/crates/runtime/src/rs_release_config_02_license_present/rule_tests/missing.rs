use g3rs_release_config_checks_assertions::rs_release_config_02_license_present as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_license_missing() {
    let toml = "\
[package]
name = \"no-license\"
version = \"1.0.0\"
edition = \"2024\"
description = \"A crate without license\"
";
    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "no-license: missing license",
            "Publishable crates must have a license or license-file field in [package].",
            "Cargo.toml",
            false,
        )],
    );
}

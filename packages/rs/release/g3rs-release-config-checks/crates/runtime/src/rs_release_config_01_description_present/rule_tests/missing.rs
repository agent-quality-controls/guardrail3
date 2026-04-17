use g3rs_release_config_checks_assertions::rs_release_config_01_description_present::rule as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_description_missing() {
    let toml = "\
[package]
name = \"no-desc\"
version = \"1.0.0\"
edition = \"2024\"
";
    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "no-desc: missing description",
            "Publishable crates must have a description field in [package].",
            "Cargo.toml",
            false,
        )],
    );
}

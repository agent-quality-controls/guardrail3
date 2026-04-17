use g3rs_release_config_checks_assertions::rs_release_config_04_keywords_present::rule as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_keywords_missing() {
    let toml = "\
[package]
name = \"no-kw\"
version = \"1.0.0\"
edition = \"2024\"
description = \"A crate without keywords\"
";
    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "no-kw: keywords missing",
            "Publishable crates must have keywords in [package].",
            "Cargo.toml",
            false,
        )],
    );
}

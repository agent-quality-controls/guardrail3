use g3rs_release_config_checks_assertions::rs_release_config_05_categories_present as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_categories_missing() {
    let toml = "\
[package]
name = \"no-cats\"
version = \"1.0.0\"
edition = \"2024\"
description = \"A crate without categories\"
";
    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "no-cats: categories missing",
            "Publishable crates must have categories in [package].",
            "Cargo.toml",
            false,
        )],
    );
}

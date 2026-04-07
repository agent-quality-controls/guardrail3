use g3rs_release_config_checks_assertions::rs_release_config_03_repository_present as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_repository_missing() {
    let toml = "\
[package]
name = \"no-repo\"
version = \"1.0.0\"
edition = \"2024\"
description = \"A crate without repository\"
";
    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "no-repo: missing repository",
            "Publishable crates must have a repository field in [package].",
            "Cargo.toml",
            false,
        )],
    );
}

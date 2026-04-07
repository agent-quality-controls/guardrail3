use g3rs_release_config_checks_assertions::rs_release_config_08_binstall_metadata as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_binstall_metadata_missing() {
    let toml = "\
[package]
name = \"no-binstall\"
version = \"1.0.0\"
edition = \"2024\"
description = \"A binary without binstall metadata\"

[[bin]]
name = \"no-binstall\"
path = \"src/main.rs\"
";
    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "no-binstall: missing binstall metadata",
            "Binary crates should have [package.metadata.binstall] for cargo-binstall support.",
            "Cargo.toml",
            false,
        )],
    );
}

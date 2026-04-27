use g3rs_release_config_checks_assertions::docs_rs_metadata::rule as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_docs_rs_metadata_missing() {
    let toml = "\
[package]
name = \"no-docs-rs\"
version = \"1.0.0\"
edition = \"2024\"
description = \"A library without docs.rs metadata\"

[lib]
path = \"src/lib.rs\"
";
    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "no-docs-rs: docs.rs metadata missing",
            "Library crates should have [package.metadata.docs.rs] for docs.rs configuration.",
            "Cargo.toml",
            false,
        )],
    );
}

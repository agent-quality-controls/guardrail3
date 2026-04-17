use g3rs_release_config_checks_assertions::rs_release_config_09_accidentally_publishable::rule as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_all_metadata_missing() {
    let toml = "\
[package]
name = \"bare-crate\"
version = \"1.0.0\"
edition = \"2024\"
";
    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "bare-crate may be accidentally publishable",
            "Crate is publishable but has no description, license, or repository. \
             If this crate is not intended for publication, add `publish = false` to [package].",
            "Cargo.toml",
            false,
        )],
    );
}

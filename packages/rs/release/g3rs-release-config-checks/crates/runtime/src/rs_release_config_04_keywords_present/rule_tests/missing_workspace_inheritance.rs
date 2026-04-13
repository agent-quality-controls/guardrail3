use g3rs_release_config_checks_assertions::rs_release_config_04_keywords_present as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_workspace_inherited_keywords_have_no_workspace_values() {
    let toml = "\
[package]
name = \"member-crate\"
version = \"0.1.0\"
edition = \"2024\"
keywords = { workspace = true }
";

    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "member-crate: keywords missing",
            "Publishable crates must have keywords in [package].",
            "Cargo.toml",
            false,
        )],
    );
}

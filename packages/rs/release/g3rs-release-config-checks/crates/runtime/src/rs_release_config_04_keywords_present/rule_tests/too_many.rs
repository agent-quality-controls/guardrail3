use g3rs_release_config_checks_assertions::rs_release_config_04_keywords_present::rule as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_too_many_keywords() {
    let toml = "\
[package]
name = \"too-many-kw\"
version = \"1.0.0\"
edition = \"2024\"
description = \"A crate with too many keywords\"
keywords = [\"a\", \"b\", \"c\", \"d\", \"e\", \"f\"]
";
    let results = run_check(toml);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "too-many-kw: keywords count invalid (6)",
            "Publishable crates must have between 1 and 5 keywords.",
            "Cargo.toml",
            false,
        )],
    );
}

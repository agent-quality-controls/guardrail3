use g3rs_release_config_checks_assertions::rs_release_config_07_docs_rs_metadata as assertions;

use super::helpers::run_check;

#[test]
fn skips_when_no_lib_section() {
    let toml = "\
[package]
name = \"bin-only\"
version = \"1.0.0\"
edition = \"2024\"
description = \"A binary-only crate\"

[[bin]]
name = \"bin-only\"
path = \"src/main.rs\"
";
    let results = run_check(toml);

    assertions::assert_no_findings(&results);
}

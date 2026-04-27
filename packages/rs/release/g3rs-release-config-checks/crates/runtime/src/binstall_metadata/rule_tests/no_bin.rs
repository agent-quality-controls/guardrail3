use g3rs_release_config_checks_assertions::binstall_metadata::rule as assertions;

use super::helpers::run_check;

#[test]
fn skips_when_no_bin_section() {
    let toml = "\
[package]
name = \"lib-only\"
version = \"1.0.0\"
edition = \"2024\"
description = \"A library-only crate\"

[lib]
path = \"src/lib.rs\"
";
    let results = run_check(toml);

    assertions::assert_no_findings(&results);
}

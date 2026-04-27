use g3rs_release_config_checks_assertions::accidentally_publishable::rule as assertions;

use super::helpers::run_check;

#[test]
fn skips_when_publish_false() {
    let toml = "\
[package]
name = \"private-crate\"
version = \"1.0.0\"
edition = \"2024\"
publish = false
";
    let results = run_check(toml);

    assertions::assert_no_findings(&results);
}

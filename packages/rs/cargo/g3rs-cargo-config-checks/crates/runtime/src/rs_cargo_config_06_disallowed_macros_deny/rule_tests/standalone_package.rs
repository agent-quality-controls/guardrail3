use g3rs_cargo_config_checks_assertions::rs_cargo_config_06_disallowed_macros_deny::rule as assertions;
use super::helpers::run_check;

const STANDALONE_PACKAGE_WITH_DENY: &str = r#"
[package]
name = "standalone"
edition = "2024"

[lints.clippy]
disallowed_macros = "deny"
"#;

#[test]
fn inventories_when_standalone_package_has_disallowed_macros_denied() {
    let results = run_check(STANDALONE_PACKAGE_WITH_DENY);
    assertions::assert_has_info(&results, "disallowed macros lint enforced", true);
}

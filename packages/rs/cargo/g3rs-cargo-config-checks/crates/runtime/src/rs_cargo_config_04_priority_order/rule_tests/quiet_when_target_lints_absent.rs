use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::rs_cargo_config_04_priority_order::rule as assertions;

#[test]
fn stays_quiet_when_specific_target_lints_are_absent() {
    let results = run_check(
        r#"
[workspace]
members = []

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
nursery = { level = "deny", priority = -1 }
cargo = { level = "deny", priority = -1 }
"#,
    );

    assertions::assert_no_findings(&results);
}

use g3rs_cargo_config_checks_assertions::rs_cargo_config_01_workspace_lints::rule as assertions;
use super::helpers::run_check;

#[test]
fn errors_when_clippy_lints_table_is_missing() {
    let results = run_check(
        r#"
[workspace]
members = []

[workspace.lints.rust]
warnings = "deny"
unsafe_code = "forbid"
dead_code = "deny"
unused_results = "deny"
unused_crate_dependencies = "deny"
missing_debug_implementations = "warn"
unreachable_pub = "deny"
"#,
    );

    assertions::assert_has_error(&results, "clippy lint table missing", false);
}

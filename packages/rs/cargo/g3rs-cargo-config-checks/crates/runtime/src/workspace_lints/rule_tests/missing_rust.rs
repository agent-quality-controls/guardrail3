use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::workspace_lints::rule as assertions;

#[test]
fn errors_when_rust_lints_table_is_missing() {
    let results = run_check(
        r#"
[workspace]
members = []

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
nursery = { level = "deny", priority = -1 }
cargo = { level = "deny", priority = -1 }
unwrap_used = "deny"
expect_used = "deny"
indexing_slicing = "deny"
panic = "deny"
todo = "deny"
unimplemented = "deny"
dbg_macro = "deny"
module_name_repetitions = "allow"
disallowed_macros = "deny"
"#,
    );

    assertions::assert_has_error(&results, "rust lint table missing", false);
}

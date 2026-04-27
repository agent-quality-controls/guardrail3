use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::workspace_lints::rule as assertions;

#[test]
fn errors_when_required_clippy_entry_is_missing() {
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

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
nursery = { level = "deny", priority = -1 }
cargo = { level = "deny", priority = -1 }
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
todo = "deny"
unimplemented = "deny"
dbg_macro = "deny"
module_name_repetitions = "allow"
disallowed_macros = "deny"
"#,
    );

    assertions::assert_has_error(&results, "missing clippy lint `indexing_slicing`", false);
}

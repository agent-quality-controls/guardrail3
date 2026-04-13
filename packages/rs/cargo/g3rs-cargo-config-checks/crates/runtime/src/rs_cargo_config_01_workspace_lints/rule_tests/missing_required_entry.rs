use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

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

    let result = results
        .iter()
        .find(|result| result.id() == "RS-CARGO-CONFIG-01")
        .unwrap();
    assert_eq!(result.severity(), G3Severity::Error);
    assert!(result.title().starts_with("missing clippy lint `"));
}

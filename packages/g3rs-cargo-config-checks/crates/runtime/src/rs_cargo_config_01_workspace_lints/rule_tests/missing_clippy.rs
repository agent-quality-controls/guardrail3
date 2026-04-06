use guardrail3_check_types::G3Severity;

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

    let result = results.iter().find(|result| result.id() == "RS-CARGO-CONFIG-01").unwrap();
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "clippy lint table missing");
}

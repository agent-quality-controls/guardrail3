use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

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

    let result = results
        .iter()
        .find(|result| result.id() == "RS-CARGO-CONFIG-01")
        .unwrap();
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "rust lint table missing");
}

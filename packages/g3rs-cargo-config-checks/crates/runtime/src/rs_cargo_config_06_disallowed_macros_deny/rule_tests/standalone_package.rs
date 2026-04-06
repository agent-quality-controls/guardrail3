use guardrail3_check_types::G3Severity;

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
    let result = results
        .iter()
        .find(|result| result.id() == "RS-CARGO-CONFIG-06")
        .expect("should produce a result for RS-CARGO-CONFIG-06");
    assert_eq!(
        result.severity(),
        G3Severity::Info,
        "standalone package with disallowed_macros = deny should pass"
    );
    assert!(result.inventory(), "successful check should be an inventory result");
}

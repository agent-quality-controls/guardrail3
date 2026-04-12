use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn reports_inventory_when_cargo_deny_is_installed() {
    let results = run_check(&["cargo-deny"]);

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(results[0].id(), "RS-DEPS-CONFIG-06");
    assert_eq!(results[0].severity(), G3Severity::Info);
    assert_eq!(results[0].title(), "cargo-deny installed");
    assert_eq!(results[0].file(), Some("Cargo.toml"));
    assert!(results[0].inventory());
}

#[test]
fn reports_error_when_cargo_deny_is_missing() {
    let results = run_check(&[]);

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(results[0].id(), "RS-DEPS-CONFIG-06");
    assert_eq!(results[0].severity(), G3Severity::Error);
    assert_eq!(results[0].title(), "cargo-deny missing");
    assert_eq!(results[0].file(), Some("Cargo.toml"));
    assert!(!results[0].inventory());
}

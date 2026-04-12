use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn reports_inventory_when_cargo_dupes_is_installed() {
    let results = run_check(&["cargo-dupes"]);

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(results[0].id(), "RS-DEPS-CONFIG-08");
    assert_eq!(results[0].severity(), G3Severity::Info);
    assert_eq!(results[0].title(), "cargo-dupes installed");
    assert_eq!(results[0].file(), Some("Cargo.toml"));
    assert!(results[0].inventory());
}

#[test]
fn reports_warning_when_cargo_dupes_is_missing() {
    let results = run_check(&[]);

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(results[0].id(), "RS-DEPS-CONFIG-08");
    assert_eq!(results[0].severity(), G3Severity::Warn);
    assert_eq!(results[0].title(), "cargo-dupes missing");
    assert_eq!(results[0].file(), Some("Cargo.toml"));
    assert!(!results[0].inventory());
}

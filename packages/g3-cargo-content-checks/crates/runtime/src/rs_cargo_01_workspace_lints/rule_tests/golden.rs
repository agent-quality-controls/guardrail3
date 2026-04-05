use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn inventories_when_required_lint_tables_are_present() {
    let results = run_check(include_str!("fixtures/golden_workspace.toml"));
    let result = results.iter().find(|result| result.id() == "RS-CARGO-01").unwrap();
    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}

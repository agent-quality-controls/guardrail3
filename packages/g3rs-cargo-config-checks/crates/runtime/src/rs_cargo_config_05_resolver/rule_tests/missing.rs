use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn errors_when_workspace_resolver_is_missing() {
    let results = run_check("[workspace]\nmembers = []\n");
    let result = results.iter().find(|result| result.id() == "RS-CARGO-CONFIG-05").unwrap();
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "workspace resolver missing");
}

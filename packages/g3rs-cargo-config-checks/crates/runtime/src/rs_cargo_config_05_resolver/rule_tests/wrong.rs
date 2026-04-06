use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn errors_when_workspace_resolver_is_unsupported() {
    let results = run_check("[workspace]\nmembers = []\nresolver = \"1\"\n");
    let result = results.iter().find(|result| result.id() == "RS-CARGO-CONFIG-05").unwrap();
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "unsupported workspace resolver");
}

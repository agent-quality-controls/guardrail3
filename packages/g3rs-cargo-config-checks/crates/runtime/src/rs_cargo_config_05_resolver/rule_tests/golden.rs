use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn inventories_when_workspace_resolver_is_supported() {
    let results = run_check("[workspace]\nmembers = []\nresolver = \"3\"\n");
    let result = results.iter().find(|result| result.id() == "RS-CARGO-CONFIG-05").unwrap();
    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}

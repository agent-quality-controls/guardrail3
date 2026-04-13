use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn inventories_when_hybrid_root_falls_back_to_package_edition() {
    let results = run_check(
        "[workspace]\nmembers = []\nresolver = \"3\"\n\n[package]\nname = \"hybrid\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    let result = results
        .iter()
        .find(|result| result.id() == "RS-CARGO-CONFIG-03")
        .unwrap();
    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}

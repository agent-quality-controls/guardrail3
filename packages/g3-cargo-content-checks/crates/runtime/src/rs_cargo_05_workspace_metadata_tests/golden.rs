use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn inventories_when_edition_is_supported() {
    let results = run_check("[package]\nname = \"pkg\"\nedition = \"2024\"\n");
    let result = results.iter().find(|result| result.id() == "RS-CARGO-05").unwrap();
    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}

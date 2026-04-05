use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn errors_when_edition_is_missing() {
    let results = run_check("[package]\nname = \"pkg\"\n");
    let result = results.iter().find(|result| result.id() == "RS-CARGO-05").unwrap();
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "edition missing");
}

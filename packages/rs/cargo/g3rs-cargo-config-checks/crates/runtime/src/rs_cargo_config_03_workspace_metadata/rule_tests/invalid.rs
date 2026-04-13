use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn errors_when_edition_is_not_a_string() {
    let results = run_check(
        "[workspace]\nmembers = []\n\n[package]\nname = \"pkg\"\nedition.workspace = true\n",
    );
    let result = results
        .iter()
        .find(|result| result.id() == "RS-CARGO-CONFIG-03")
        .unwrap();
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "edition invalid");
}

use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn errors_when_library_only_rust_lint_is_missing() {
    let results = run_check(
        include_str!("fixtures/golden_workspace.toml")
            .replace("unreachable_pub = \"deny\"\n", "")
            .as_str(),
    );
    let result = results
        .iter()
        .find(|result| result.title() == "missing rust lint `unreachable_pub`")
        .unwrap();
    assert_eq!(result.severity(), G3Severity::Error);
}

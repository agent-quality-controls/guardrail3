use g3rs_arch_types::types::G3RsArchRustPolicyState;
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_has_result(
    results: &[G3CheckResult],
    id: &str,
    severity: G3Severity,
    file: Option<&str>,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == id
                && result.severity() == severity
                && file.is_none_or(|expected| result.file() == Some(expected))
        }),
        "{results:#?}"
    );
}

pub fn assert_missing_result(results: &[G3CheckResult], id: &str) {
    assert!(
        !results.iter().any(|result| result.id() == id),
        "{results:#?}"
    );
}

pub fn assert_parsed_rust_policy(
    rust_policy: &G3RsArchRustPolicyState,
    expected_rule: &str,
    expected_file: &str,
    expected_selector: &str,
) {
    let G3RsArchRustPolicyState::Parsed { waivers, .. } = rust_policy else {
        assert!(false, "expected parsed rust policy, got {rust_policy:#?}");
        return;
    };

    assert_eq!(waivers.len(), 1, "{waivers:#?}");
    assert_eq!(waivers[0].rule, expected_rule);
    assert_eq!(waivers[0].file, expected_file);
    assert_eq!(waivers[0].selector, expected_selector);
}

#![expect(
    clippy::indexing_slicing,
    reason = "assertion helper requires direct indexing of expected fixed-size finding slice"
)]
crate::define_rule_assertions!("g3rs-hooks/required-contract-command-present");

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_single_inventory(
    results: &[guardrail3_check_types::G3CheckResult],
    label: &str,
    owners: &str,
) {
    let owner_message = format!("Owner families: {owners}");
    assert_eq!(results.len(), 1, "expected exactly one contract finding");
    let result = &results[0];
    assert_eq!(
        result.id(),
        "g3rs-hooks/required-contract-command-present",
        "assertion failed"
    );
    assert_eq!(result.severity(), Severity::Info, "assertion failed");
    assert!(result.inventory(), "valid command should emit inventory");
    assert_eq!(
        result.title(),
        "hook contract command is present",
        "assertion failed"
    );
    assert!(
        result.message().contains(label),
        "finding should include command label `{label}`: {}",
        result.message()
    );
    assert!(
        result.message().contains(owner_message.as_str()),
        "finding should include owner families `{owners}`: {}",
        result.message()
    );
}

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_missing(
    results: &[guardrail3_check_types::G3CheckResult],
    label: &str,
    owners: &str,
) {
    let owner_message = format!("Owner families: {owners}");
    assert_eq!(results.len(), 1, "expected exactly one contract finding");
    let result = &results[0];
    assert_eq!(
        result.id(),
        "g3rs-hooks/required-contract-command-present",
        "assertion failed"
    );
    assert_eq!(result.severity(), Severity::Error, "assertion failed");
    assert!(
        !result.inventory(),
        "missing command should not be inventory"
    );
    assert_eq!(
        result.title(),
        "hook contract command is missing",
        "assertion failed"
    );
    assert!(
        result.message().contains(label),
        "finding should include command label `{label}`: {}",
        result.message()
    );
    assert!(
        result.message().contains(owner_message.as_str()),
        "finding should include owner families `{owners}`: {}",
        result.message()
    );
}

/// `family_findings` function.
fn family_findings<'a>(
    results: &'a [guardrail3_check_types::G3CheckResult],
    owners: &str,
) -> Vec<&'a guardrail3_check_types::G3CheckResult> {
    let owner_message = format!("Owner families: {owners}");
    results
        .iter()
        .filter(|result| {
            result.id() == "g3rs-hooks/required-contract-command-present"
                && result.message().contains(&owner_message)
        })
        .collect()
}

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_family_delegated_inventory(
    results: &[guardrail3_check_types::G3CheckResult],
    owners: &str,
    expected_count: usize,
) {
    let findings = family_findings(results, owners);
    assert_eq!(
        findings.len(),
        expected_count,
        "expected one contract result per family-delegated command; got {findings:#?}",
    );
    for finding in &findings {
        assert!(
            finding.inventory(),
            "delegated command should be inventory, not finding; got {finding:#?}",
        );
        assert!(
            finding
                .title()
                .contains("delegated to g3rs validate --staged")
                || finding
                    .message()
                    .contains("delegates to per-unit `g3rs validate --path"),
            "delegated inventory should describe validate-staged delegation; got {finding:#?}",
        );
    }
}

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_family_missing_findings(
    results: &[guardrail3_check_types::G3CheckResult],
    owners: &str,
    expected_count: usize,
) {
    let findings = family_findings(results, owners);
    assert_eq!(
        findings.len(),
        expected_count,
        "expected one contract result per family-delegated command; got {findings:#?}",
    );
    for finding in &findings {
        assert!(
            !finding.inventory(),
            "missing command should produce non-inventory finding; got {finding:#?}",
        );
        assert!(
            finding.title().contains("missing"),
            "non-delegating hook should warn missing; got {finding:#?}",
        );
    }
}

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_family_inline_inventory(
    results: &[guardrail3_check_types::G3CheckResult],
    owners: &str,
) {
    let findings = family_findings(results, owners);
    assert_eq!(findings.len(), 1, "expected exactly one contract finding");
    assert!(
        findings[0].inventory(),
        "requirement should be satisfied by inline command; got {:#?}",
        findings[0]
    );
}

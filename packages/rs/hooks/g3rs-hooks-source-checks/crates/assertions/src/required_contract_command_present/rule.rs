crate::define_rule_assertions!("g3rs-hooks/required-contract-command-present");

pub fn assert_single_inventory(
    results: &[guardrail3_check_types::G3CheckResult],
    label: &str,
    owners: &str,
) {
    let owner_message = format!("Owner families: {owners}");
    assert_eq!(results.len(), 1, "expected exactly one contract finding");
    let result = &results[0];
    assert_eq!(result.id(), "g3rs-hooks/required-contract-command-present");
    assert_eq!(result.severity(), Severity::Info);
    assert!(result.inventory(), "valid command should emit inventory");
    assert_eq!(result.title(), "hook contract command is present");
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

pub fn assert_missing(
    results: &[guardrail3_check_types::G3CheckResult],
    label: &str,
    owners: &str,
) {
    let owner_message = format!("Owner families: {owners}");
    assert_eq!(results.len(), 1, "expected exactly one contract finding");
    let result = &results[0];
    assert_eq!(result.id(), "g3rs-hooks/required-contract-command-present");
    assert_eq!(result.severity(), Severity::Warn);
    assert!(
        !result.inventory(),
        "missing command should not be inventory"
    );
    assert_eq!(result.title(), "hook contract command is missing");
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

crate::define_rule_assertions!("RS-RELEASE-12");

pub const RELEASE_PLZ_EXISTS_RULE_ID: &str = "RS-RELEASE-02";
pub const RELEASE_PLZ_COVERAGE_RULE_ID: &str = "RS-RELEASE-03";
pub const CLIFF_EXISTS_RULE_ID: &str = "RS-RELEASE-04";
pub const RELEASE_WORKFLOW_RULE_ID: &str = "RS-RELEASE-05";
pub const PUBLISH_DRY_RUN_WORKFLOW_RULE_ID: &str = "RS-RELEASE-06";
pub const REGISTRY_TOKEN_RULE_ID: &str = "RS-RELEASE-07";
pub const README_QUALITY_RULE_ID: &str = "RS-PUB-05";

pub fn assert_related_rule_file_absent(results: &[CheckResult], rule_id: &str, file: &str) {
    assert!(
        !results
            .iter()
            .any(|result| result.id == rule_id && result.file.as_deref() == Some(file)),
        "unexpected {rule_id} finding for {file}: {results:#?}"
    );
}

pub fn assert_rule_file_absent(results: &[CheckResult], file: &str) {
    assert!(
        !results
            .iter()
            .any(|result| result.id == RULE_ID && result.file.as_deref() == Some(file)),
        "unexpected {RULE_ID} finding for {file}: {results:#?}"
    );
}

pub fn assert_unreadable_cached_files_fail_closed(results: &[CheckResult]) {
    let actual = findings(results);
    let expected = [
        ("Cargo.toml", "Failed to read Cargo.toml"),
        ("crates/example/Cargo.toml", "Failed to read Cargo.toml"),
        ("release-plz.toml", "Failed to read release-plz.toml"),
        ("cliff.toml", "Failed to read cliff.toml"),
        (
            ".github/workflows/release.yml",
            "Failed to read workflow YAML",
        ),
    ];

    for (file, needle) in expected {
        assert!(
            actual.iter().any(|result| {
                result.file.as_deref() == Some(file) && result.message.contains(needle)
            }),
            "missing RS-RELEASE-12 fail-closed finding for {file}: {actual:#?}"
        );
    }
}

pub fn assert_related_rule_results(
    results: &[guardrail3_domain_report::CheckResult],
    rule_id: &str,
    expected: &[ExpectedRuleResult<'_>],
) {
    let actual = results
        .iter()
        .filter(|result| result.id == rule_id)
        .collect::<Vec<_>>();
    assert_eq!(
        actual.len(),
        expected.len(),
        "unexpected {rule_id} results: {actual:#?}"
    );

    for expected_result in expected {
        let matched = actual.iter().any(|result| {
            expected_result
                .severity
                .is_none_or(|severity| result.severity == severity)
                && expected_result
                    .title
                    .is_none_or(|title| result.title == title)
                && expected_result
                    .title_contains
                    .is_none_or(|needle| result.title.contains(needle))
                && expected_result
                    .file
                    .is_none_or(|file| result.file.as_deref() == Some(file))
                && expected_result
                    .inventory
                    .is_none_or(|inventory| result.inventory == inventory)
                && expected_result
                    .message
                    .is_none_or(|message| result.message == message)
                && expected_result
                    .message_contains
                    .is_none_or(|needle| result.message.contains(needle))
        });
        assert!(
            matched,
            "missing expected {rule_id} result: {expected_result:#?}\nactual: {actual:#?}"
        );
    }
}

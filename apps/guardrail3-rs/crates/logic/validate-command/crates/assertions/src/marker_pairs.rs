use guardrail3_check_types::G3CheckResult;

/// Checks that marker-pair validation returned no findings.
///
/// # Panics
///
/// Panics when any marker-pair finding is present.
pub fn assert_no_marker_pair_findings(results: &[G3CheckResult]) {
    assert!(
        results.is_empty(),
        "fixture marker pairs must not affect repo validation: {results:#?}"
    );
}

/// Checks the incomplete-adoption marker-pair finding shape.
///
/// # Panics
///
/// Panics when the marker-pair finding count, id, or file path does not match.
pub fn assert_incomplete_adoption_marker_pair(results: &[G3CheckResult], expected_file: &str) {
    assert_eq!(results.len(), 1, "expected one marker-pair finding");
    let Some(result) = results.first() else {
        return;
    };
    assert_eq!(
        result.id(),
        "g3rs-topology/marker-pair-incomplete",
        "marker-pair finding id should stay stable"
    );
    assert_eq!(
        result.file(),
        Some(expected_file),
        "marker-pair finding file should point at the incomplete marker"
    );
}

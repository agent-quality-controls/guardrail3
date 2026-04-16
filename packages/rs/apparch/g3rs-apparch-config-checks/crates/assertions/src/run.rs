use guardrail3_check_types::G3CheckResult;

pub fn assert_no_finding_for_file(results: &[G3CheckResult], file: &str) {
    assert!(
        results.iter().all(|result| result.file() != Some(file)),
        "{results:#?}"
    );
}

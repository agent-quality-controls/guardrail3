#[must_use]
pub fn find_root<'a>(
    inputs: &'a [g3rs_test_types::G3RsTestConfigChecksInput],
    root_rel_dir: &str,
) -> Option<&'a g3rs_test_types::G3RsTestConfigChecksInput> {
    inputs
        .iter()
        .find(|candidate| candidate.root_rel_dir == root_rel_dir)
}

pub fn assert_result(
    results: &[guardrail3_check_types::G3CheckResult],
    id: &str,
    title: &str,
    file: Option<&str>,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == id && result.title() == title && result.file() == file
        }),
        "{results:#?}"
    );
}

pub fn assert_file_has_result(
    results: &[guardrail3_check_types::G3CheckResult],
    file: &str,
    id: &str,
) {
    assert!(
        results
            .iter()
            .any(|result| result.file() == Some(file) && result.id() == id),
        "{results:#?}"
    );
}

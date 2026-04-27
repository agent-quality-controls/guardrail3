use guardrail3_check_types::G3CheckResult;

pub fn assert_no_results(results: &[G3CheckResult]) {
    assert!(results.is_empty(), "{results:#?}");
}

pub fn assert_rule_present(results: &[G3CheckResult], id: &str, file: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == id && result.file() == Some(file)),
        "{results:#?}"
    );
}

pub fn assert_rule_absent(results: &[G3CheckResult], id: &str, title: &str) {
    assert!(
        results
            .iter()
            .all(|result| !(result.id() == id && result.title() == title)),
        "{results:#?}"
    );
}

pub fn assert_rule_id_absent(results: &[G3CheckResult], id: &str) {
    assert!(
        results.iter().all(|result| result.id() != id),
        "{results:#?}"
    );
}

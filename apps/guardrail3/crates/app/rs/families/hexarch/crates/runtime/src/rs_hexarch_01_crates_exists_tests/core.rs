use guardrail3_domain_report::Severity;

use super::super::check_with_top_level_entries_for_tests;

#[test]
fn passes_when_top_level_crates_dir_has_entries() {
    let results = check_with_top_level_entries_for_tests(1);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn fails_when_top_level_crates_dir_has_no_entries() {
    let results = check_with_top_level_entries_for_tests(0);

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(results[0].id, "");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("apps/backend"));
    assert!(results[0].title.contains("missing crates/"), "{results:#?}");
}

use std::collections::BTreeSet;

use guardrail3_domain_report::CheckResult;

pub fn assert_result_summary<I>(
    results: &[&CheckResult],
    expected_count: usize,
    expected_files: I,
    expected_file: Option<Option<&str>>,
    title_contains: Option<&str>,
    message_contains: Option<&str>,
) where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    assert_eq!(results.len(), expected_count, "{results:#?}");

    let actual_files = results
        .iter()
        .filter_map(|result| result.file.as_deref())
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let expected_files = expected_files
        .into_iter()
        .map(|file| file.as_ref().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_files, expected_files, "{results:#?}");

    if let Some(expected_file) = expected_file {
        assert_eq!(results.len(), 1, "{results:#?}");
        assert_eq!(results[0].file.as_deref(), expected_file, "{results:#?}");
    }

    if let Some(title_contains) = title_contains {
        assert!(
            results
                .iter()
                .all(|result| result.title.contains(title_contains)),
            "{results:#?}"
        );
    }

    if let Some(message_contains) = message_contains {
        assert!(
            results
                .iter()
                .all(|result| result.message.contains(message_contains)),
            "{results:#?}"
        );
    }
}

pub fn assert_result_titles<I>(results: &[&CheckResult], expected_titles: I)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let actual_titles = results
        .iter()
        .map(|result| result.title.as_str())
        .collect::<BTreeSet<_>>();
    let expected_titles = expected_titles
        .into_iter()
        .map(|title| title.as_ref().to_owned())
        .collect::<BTreeSet<_>>();
    let actual_titles = actual_titles
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_titles, expected_titles, "{results:#?}");
}

pub fn assert_result_titles_excluding(results: &[&CheckResult], forbidden_substring: &str) {
    assert!(
        results
            .iter()
            .all(|result| !result.title.contains(forbidden_substring)),
        "{results:#?}"
    );
}

pub fn assert_result_messages<I>(results: &[&CheckResult], expected_messages: I)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let actual_messages = results
        .iter()
        .map(|result| result.message.as_str())
        .collect::<BTreeSet<_>>();
    let expected_messages = expected_messages
        .into_iter()
        .map(|message| message.as_ref().to_owned())
        .collect::<BTreeSet<_>>();
    let actual_messages = actual_messages
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_messages, expected_messages, "{results:#?}");
}

pub fn assert_all_inventory(results: &[&CheckResult]) {
    assert!(
        results.iter().all(|result| result.inventory),
        "{results:#?}"
    );
}

pub fn assert_all_titles_contain(results: &[&CheckResult], required_substrings: &[&str]) {
    assert!(
        results.iter().all(|result| {
            required_substrings
                .iter()
                .all(|needle| result.title.contains(needle))
        }),
        "{results:#?}"
    );
}

pub fn count_titles_containing_all(
    results: &[&CheckResult],
    required_substrings: &[&str],
) -> usize {
    results
        .iter()
        .filter(|result| {
            required_substrings
                .iter()
                .all(|needle| result.title.contains(needle))
        })
        .count()
}

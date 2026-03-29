use std::collections::BTreeSet;

use guardrail3_domain_modules::clippy::{
    BASE_TYPE_PATHS, EXPECTED_MACRO_BANS, SERVICE_METHOD_PATHS, THRESHOLD_VALUES,
};
use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-19";

pub fn assert_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = expected
        .iter()
        .map(|message| (*message).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_messages, expected_messages);
    assert_eq!(results.len(), expected_messages.len());
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.severity == Severity::Warn
            && result.title == "unrecognized clippy.toml key"
            && result.file.as_deref() == Some(file)
    }));
}

pub fn assert_generated_top_level_keys_are_all_known_managed_keys(clippy_toml: &str) {
    let parsed = toml::from_str::<toml::Value>(clippy_toml).expect("valid clippy TOML");
    let table = parsed.as_table().expect("top-level clippy table");
    let known: BTreeSet<_> = THRESHOLD_VALUES
        .iter()
        .map(|(key, _)| *key)
        .chain([
            "avoid-breaking-exported-api",
            "allow-dbg-in-tests",
            "allow-expect-in-tests",
            "allow-panic-in-tests",
            "allow-print-in-tests",
            "allow-unwrap-in-tests",
            "disallowed-methods",
            "disallowed-types",
            "disallowed-macros",
        ])
        .collect();

    for key in table.keys() {
        assert!(
            known.contains(key.as_str()),
            "unexpected canonical top-level key: {key}"
        );
        assert!(
            known
                .iter()
                .copied()
                .filter(|managed| *managed != key.as_str())
                .all(|managed| normalized_key_distance(key, managed) > 2),
            "canonical key {key} should not look like a typo of another managed key"
        );
    }

    assert!(!SERVICE_METHOD_PATHS.is_empty());
    assert!(!BASE_TYPE_PATHS.is_empty());
    assert!(!EXPECTED_MACRO_BANS.is_empty());
}

fn normalized_key_distance(a: &str, b: &str) -> usize {
    let a = a.replace(['-', '_'], "");
    let b = b.replace(['-', '_'], "");
    levenshtein(a.as_bytes(), b.as_bytes())
}

fn levenshtein(a: &[u8], b: &[u8]) -> usize {
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }

    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut curr = vec![0usize; b.len() + 1];

    for (i, a_byte) in a.iter().enumerate() {
        curr[0] = i + 1;
        for (j, b_byte) in b.iter().enumerate() {
            let cost = usize::from(a_byte != b_byte);
            curr[j + 1] = (curr[j] + 1).min(prev[j + 1] + 1).min(prev[j] + cost);
        }
        prev.clone_from(&curr);
    }

    prev[b.len()]
}

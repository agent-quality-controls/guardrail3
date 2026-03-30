use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};
const ID: &str = "RS-CLIPPY-19";

pub fn managed_top_level_keys() -> BTreeSet<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::known_top_level_keys()
        .into_iter()
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
        .collect()
}

pub fn assert_inventory(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id(), ID);
    assert!(result.inventory());
    assert_eq!(result.severity(), Severity::Info);
    assert_eq!(result.title(), "no suspicious managed-key typos");
    assert_eq!(
        result.message(),
        "No top-level keys look like typos of guardrail-managed clippy keys."
    );
    assert_eq!(result.file(), Some(file));
}

pub fn assert_known_managed_keys_exact(parsed: &toml::Value) {
    let table = parsed.as_table().expect("top-level clippy table");
    let known = managed_top_level_keys();

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

    assert!(known.contains("disallowed-methods"));
    assert!(known.contains("disallowed-types"));
    assert!(known.contains("disallowed-macros"));
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

pub fn assert_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    let mut actual_messages = results
        .iter()
        .map(|result| result.message())
        .collect::<Vec<_>>();
    let mut expected_messages = expected.to_vec();

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert_eq!(results.len(), expected_messages.len());
    assert!(results.iter().all(|result| {
        result.id() == ID
            && result.severity() == Severity::Warn
            && result.title() == "unrecognized clippy.toml key"
            && result.file() == Some(file)
    }));
}

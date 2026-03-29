use std::collections::BTreeSet;

use test_support::build_fixture_clippy_toml;

#[test]
fn generated_top_level_keys_are_all_known_managed_keys() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", ""))
            .expect("valid clippy TOML");
    let table = parsed.as_table().expect("top-level clippy table");
    let known: BTreeSet<_> = [
        "max-struct-bools",
        "max-fn-params-bools",
        "too-many-lines-threshold",
        "too-many-arguments-threshold",
        "excessive-nesting-threshold",
        "cognitive-complexity-threshold",
        "type-complexity-threshold",
    ]
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

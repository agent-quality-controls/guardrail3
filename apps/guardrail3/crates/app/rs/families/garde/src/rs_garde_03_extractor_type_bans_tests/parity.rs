use std::collections::BTreeSet;

use super::super::super::garde_support::EXTRACTOR_TYPE_BANS;
use guardrail3_domain_modules::clippy::build_clippy_toml;

fn disallowed_type_paths(parsed: &toml::Value) -> BTreeSet<String> {
    parsed
        .get("disallowed-types")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            entry
                .get("path")
                .and_then(toml::Value::as_str)
                .map(str::to_owned)
                .or_else(|| entry.as_str().map(str::to_owned))
        })
        .collect()
}

#[test]
fn generated_service_clippy_baseline_contains_all_garde_extractor_bans() {
    let parsed = toml::from_str::<toml::Value>(&build_clippy_toml("service", false, true, "", ""))
        .expect("valid clippy TOML");
    let actual = disallowed_type_paths(&parsed);
    let expected = EXTRACTOR_TYPE_BANS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();

    assert!(expected.is_subset(&actual));
}

#[test]
fn generated_library_clippy_baseline_contains_all_garde_extractor_bans() {
    let parsed = toml::from_str::<toml::Value>(&build_clippy_toml("library", false, true, "", ""))
        .expect("valid clippy TOML");
    let actual = disallowed_type_paths(&parsed);
    let expected = EXTRACTOR_TYPE_BANS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();

    assert!(expected.is_subset(&actual));
}

use std::collections::BTreeSet;

use guardrail3_app_rs_family_garde_assertions::rs_garde_config_03_extractor_type_bans::EXTRACTOR_TYPE_BANS;

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
    let parsed = toml::from_str::<toml::Value>(&super::helpers::canonical_clippy_toml())
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
    let parsed = toml::from_str::<toml::Value>(&super::helpers::canonical_library_clippy_toml())
        .expect("valid clippy TOML");
    let actual = disallowed_type_paths(&parsed);
    let expected = EXTRACTOR_TYPE_BANS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();

    assert!(expected.is_subset(&actual));
}

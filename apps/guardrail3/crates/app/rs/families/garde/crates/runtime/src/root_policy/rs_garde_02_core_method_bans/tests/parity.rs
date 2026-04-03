use std::collections::BTreeSet;

use guardrail3_app_rs_family_garde_assertions::rs_garde_02_core_method_bans::CORE_METHOD_BANS;

fn disallowed_method_paths(parsed: &toml::Value) -> BTreeSet<String> {
    parsed
        .get("disallowed-methods")
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
fn generated_service_clippy_baseline_contains_all_core_garde_method_bans() {
    let parsed = toml::from_str::<toml::Value>(&super::helpers::canonical_clippy_toml())
        .expect("valid clippy TOML");
    let actual = disallowed_method_paths(&parsed);
    let expected = CORE_METHOD_BANS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();

    assert!(expected.is_subset(&actual));
}

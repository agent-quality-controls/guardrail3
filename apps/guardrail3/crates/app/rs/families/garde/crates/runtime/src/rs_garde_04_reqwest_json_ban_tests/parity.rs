use std::collections::BTreeSet;

use guardrail3_app_rs_family_garde_assertions::rs_garde_04_reqwest_json_ban::REQWEST_JSON_BAN;
use guardrail3_domain_modules::clippy::build_clippy_toml;

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
fn generated_service_clippy_baseline_contains_reqwest_garde_ban() {
    let parsed = toml::from_str::<toml::Value>(&build_clippy_toml("service", false, true, "", ""))
        .expect("valid clippy TOML");
    let actual = disallowed_method_paths(&parsed);

    assert!(actual.contains(REQWEST_JSON_BAN));
}

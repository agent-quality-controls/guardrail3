use std::collections::BTreeSet;

use guardrail3_app_rs_family_clippy_assertions::rs_clippy_06_extra_method_ban as assertions;
use test_support::{build_fixture_clippy_toml, garde_disabled_root_tree};

use super::super::run_for_tests;

#[test]
fn inventories_garde_owned_method_bans_as_project_specific_when_garde_is_disabled() {
    let tree = garde_disabled_root_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    let expected = extra_messages(
        &build_fixture_clippy_toml("service", false, true, "", ""),
        &build_fixture_clippy_toml("service", false, false, "", ""),
        "disallowed-methods",
    );
    let expected_refs = expected.iter().map(String::as_str).collect::<Vec<_>>();
    assertions::assert_messages(&results, &expected_refs, "clippy.toml");
}

fn extra_messages(full: &str, baseline: &str, key: &str) -> Vec<String> {
    let full_paths = paths(full, key);
    let baseline_paths = paths(baseline, key);
    full_paths
        .difference(&baseline_paths)
        .map(|path| format!("Additional method ban `{path}` beyond baseline."))
        .collect()
}

fn paths(toml_text: &str, key: &str) -> BTreeSet<String> {
    toml::from_str::<toml::Value>(toml_text)
        .ok()
        .and_then(|parsed| parsed.get(key).and_then(toml::Value::as_array).cloned())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|entry| entry.get("path").and_then(toml::Value::as_str).map(str::to_owned))
        .collect()
}

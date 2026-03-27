use std::collections::BTreeSet;

use super::super::super::clippy_support::{expected_type_bans, parse_ban_entries};
use super::super::super::test_support::build_fixture_clippy_toml;
use guardrail3_domain_modules::clippy::build_clippy_toml;

#[test]
fn generated_service_types_do_not_contain_project_specific_extras() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", "")).expect("valid clippy TOML");
    let actual = parse_ban_entries(&parsed, "disallowed-types")
        .into_iter()
        .map(|entry| entry.path)
        .collect::<BTreeSet<_>>();
    let expected = expected_type_bans(None, true)
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}

#[test]
fn generated_library_types_do_not_misclassify_managed_global_state_entries_as_extra() {
    let parsed = toml::from_str::<toml::Value>(&build_clippy_toml("library", false, true, "", ""))
        .expect("valid clippy TOML");
    let actual = parse_ban_entries(&parsed, "disallowed-types")
        .into_iter()
        .map(|entry| entry.path)
        .collect::<BTreeSet<_>>();
    let expected = expected_type_bans(Some("library"), true)
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}

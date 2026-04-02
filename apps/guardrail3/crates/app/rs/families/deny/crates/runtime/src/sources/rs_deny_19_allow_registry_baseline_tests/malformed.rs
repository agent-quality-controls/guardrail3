use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_19_allow_registry_baseline as assertions;

use super::super::build_fixture_deny_toml;

#[test]
fn errors_when_allow_registry_container_is_not_an_array() {
    let results = super::super::run_check(&build_fixture_deny_toml("service").replace(
        "allow-registry = [\"sparse+https://index.crates.io/\"]",
        "allow-registry = \"sparse+https://index.crates.io/\"",
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "malformed allow-registry container",
            "`deny.toml` must use an array for `[sources].allow-registry` entries.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn errors_when_allow_registry_contains_non_string_entries_even_if_crates_io_is_present() {
    let results = super::super::run_check(&build_fixture_deny_toml("service").replace(
        "allow-registry = [\"sparse+https://index.crates.io/\"]",
        "allow-registry = [\"sparse+https://index.crates.io/\", 123]",
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "registry allow entry must be a string",
            "`deny.toml` has non-string `[sources].allow-registry` entry at index 1.",
            "deny.toml",
            false,
        )],
    );
}

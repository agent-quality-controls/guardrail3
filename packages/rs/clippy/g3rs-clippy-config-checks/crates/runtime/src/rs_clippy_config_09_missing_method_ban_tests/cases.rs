use crate::rs_clippy_config_09_missing_method_ban::check;
use crate::support::expected_method_bans;
use crate::test_support::{findings, input_from_raw, parse_error_rust_policy, parsed_rust_policy};

#[test]
fn reports_missing_baseline_method_ban() {
    let input = input_from_raw("clippy.toml", "disallowed-methods = []\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    let findings = findings(&results);
    let missing = findings
        .iter()
        .filter(|finding| finding.title == "missing method ban")
        .collect::<Vec<_>>();

    assert_eq!(
        missing.len(),
        expected_method_bans(true).len(),
        "{findings:#?}"
    );
    assert!(
        missing
            .iter()
            .any(|finding| finding.message.contains("serde_json::from_str"))
    );
}

#[test]
fn stands_down_when_policy_context_is_invalid() {
    let input = crate::test_support::input_with_raw(
        "clippy.toml",
        "disallowed-methods = []\n",
        parse_error_rust_policy("guardrail3-rs.toml", "bad profile"),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn reports_malformed_method_section() {
    let input = crate::test_support::input_with_raw(
        "clippy.toml",
        "disallowed-methods = [1]\n",
        parsed_rust_policy(
            "guardrail3-rs.toml",
            Some(guardrail3_rs_toml_parser::RustProfile::Service),
            true,
        ),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "disallowed-methods section malformed"
            && finding.message.contains("disallowed-methods[0]")
    }));
}

#[test]
fn drops_garde_owned_method_bans_when_garde_is_disabled() {
    let input = crate::test_support::input_with_raw(
        "clippy.toml",
        "disallowed-methods = []\n",
        parsed_rust_policy(
            "guardrail3-rs.toml",
            Some(guardrail3_rs_toml_parser::RustProfile::Service),
            false,
        ),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    let findings = findings(&results);
    let missing = findings
        .iter()
        .filter(|finding| finding.title == "missing method ban")
        .collect::<Vec<_>>();
    assert_eq!(
        missing.len(),
        expected_method_bans(false).len(),
        "{findings:#?}"
    );
}

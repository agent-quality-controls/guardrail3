use crate::missing_method_ban::check;
use g3rs_clippy_config_checks_assertions::missing_method_ban as assertions;
use test_support::{
    input_from_raw, input_with_raw, missing_cargo_root, parse_error_rust_policy, parsed_rust_policy,
};

#[test]
fn reports_missing_baseline_method_ban() {
    let input = input_from_raw("clippy.toml", "disallowed-methods = []\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_missing_method_ban_count(&results, 70);
    assertions::assert_contains_for_path(&results, "serde_json::from_str");
}

#[test]
fn stands_down_when_policy_context_is_invalid() {
    let input = input_with_raw(
        "clippy.toml",
        "disallowed-methods = []\n",
        parse_error_rust_policy("guardrail3-rs.toml", "bad profile"),
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_no_findings(&results);
}

#[test]
fn reports_malformed_method_section() {
    let input = input_with_raw(
        "clippy.toml",
        "disallowed-methods = [1]\n",
        parsed_rust_policy(
            "guardrail3-rs.toml",
            Some(guardrail3_rs_toml_parser::types::RustProfile::Service),
            true,
        ),
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_contains_malformed_section(&results, "disallowed-methods[0]");
}

#[test]
fn drops_garde_owned_method_bans_when_garde_is_disabled() {
    let input = input_with_raw(
        "clippy.toml",
        "disallowed-methods = []\n",
        parsed_rust_policy(
            "guardrail3-rs.toml",
            Some(guardrail3_rs_toml_parser::types::RustProfile::Service),
            false,
        ),
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_missing_method_ban_count(&results, 26);
}

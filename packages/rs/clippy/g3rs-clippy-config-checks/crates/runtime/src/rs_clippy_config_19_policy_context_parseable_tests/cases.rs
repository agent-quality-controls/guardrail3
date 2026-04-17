use crate::rs_clippy_config_19_policy_context_parseable::check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_19_policy_context_parseable as assertions;
use guardrail3_rs_toml_parser::types::RustProfile;
use test_support::{
    input_with_raw, missing_cargo_root, parse_error_rust_policy, parsed_rust_policy,
};

#[test]
fn reports_policy_context_parse_errors() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        parse_error_rust_policy("guardrail3-rs.toml", "bad profile"),
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        assertions::findings(&results),
        vec![assertions::error(
            "clippy rust policy is not parseable",
            "Failed to parse active `guardrail3-rs.toml` used for clippy profile and garde policy: bad profile",
            "guardrail3-rs.toml",
            false,
        )]
    );
}

#[test]
fn inventories_parseable_policy_context() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Service), true),
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "clippy rust policy parseable",
            "Active `guardrail3-rs.toml` parsed successfully for clippy policy context.",
            "guardrail3-rs.toml",
            true,
        )],
    );
}

use crate::rs_clippy_config_19_policy_context_parseable::check;
use crate::test_support::{findings, input_with_raw, parse_error_rust_policy, parsed_rust_policy};
use guardrail3_rs_toml_parser::RustProfile;

#[test]
fn reports_policy_context_parse_errors() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        parse_error_rust_policy("guardrail3-rs.toml", "bad profile"),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        findings(&results),
        vec![crate::test_support::Finding {
            id: "RS-CLIPPY-CONFIG-19".to_owned(),
            severity: guardrail3_check_types::G3Severity::Error,
            title: "clippy rust policy is not parseable".to_owned(),
            message: "Failed to parse active `guardrail3-rs.toml` used for clippy profile and garde policy: bad profile".to_owned(),
            file: Some("guardrail3-rs.toml".to_owned()),
            inventory: false,
        }]
    );
}

#[test]
fn inventories_parseable_policy_context() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Service), true),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(
        findings(&results).iter().any(|finding| {
            finding.title == "clippy rust policy parseable" && finding.inventory
        })
    );
}

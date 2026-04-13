use super::check;
use crate::test_support::{findings, input_with_raw, parse_error_policy, parsed_policy};

#[test]
fn reports_policy_context_parse_errors() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        parse_error_policy("guardrail3.toml", "bad profile"),
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
            title: "clippy policy context is not parseable".to_owned(),
            message: "Failed to parse active `guardrail3.toml` used for clippy profile and garde policy: bad profile".to_owned(),
            file: Some("guardrail3.toml".to_owned()),
            inventory: false,
        }]
    );
}

#[test]
fn inventories_parseable_policy_context() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        parsed_policy("guardrail3.toml", Some("service"), true),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "clippy policy context parseable" && finding.inventory
    }));
}

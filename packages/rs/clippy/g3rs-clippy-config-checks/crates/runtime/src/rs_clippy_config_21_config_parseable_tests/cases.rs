use crate::rs_clippy_config_21_config_parseable::check;
use crate::test_support::{baseline_toml, findings, input_from_raw};
use guardrail3_rs_toml_parser::RustProfile;

#[test]
fn inventories_parseable_clippy_toml() {
    let input = input_from_raw("clippy.toml", &baseline_toml(RustProfile::Service, true));
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(
        findings(&results)
            .iter()
            .any(|finding| { finding.title == "clippy.toml parseable" && finding.inventory })
    );
}

#[test]
fn errors_on_typed_parse_failures() {
    let input = input_from_raw("clippy.toml", "disallowed-methods = [{ path = 7 }]\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "clippy.toml parse error"
            && finding.message.contains("Failed to parse `clippy.toml`")
    }));
}

#[test]
fn errors_on_raw_toml_parse_failures() {
    let input = input_from_raw("clippy.toml", "not = [valid");
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "clippy.toml parse error"
            && finding.message.contains("Failed to parse `clippy.toml`")
    }));
}

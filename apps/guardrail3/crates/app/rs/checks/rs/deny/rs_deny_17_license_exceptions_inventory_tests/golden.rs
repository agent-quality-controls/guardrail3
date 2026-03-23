use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::super::check;

#[test]
fn emits_no_result_when_no_license_exceptions_exist() {
    let config = config_facts(&canonical_deny_toml_service());
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(
        results.is_empty(),
        "expected canonical license config to avoid exceptions inventory: {results:#?}"
    );
}

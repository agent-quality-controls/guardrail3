use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::super::check;

#[test]
fn emits_no_result_for_canonical_non_duplicated_entries() {
    let config = config_facts(&canonical_deny_toml_service());
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(
        results.is_empty(),
        "expected canonical deny config to avoid duplicate warnings: {results:#?}"
    );
}

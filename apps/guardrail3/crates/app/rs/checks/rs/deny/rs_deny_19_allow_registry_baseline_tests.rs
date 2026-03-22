use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn errors_on_unexpected_registry() {
    let config = config_facts(&canonical_deny_toml_service().replace(
        "allow-registry = [\"https://github.com/rust-lang/crates.io-index\"]",
        "allow-registry = [\"https://github.com/rust-lang/crates.io-index\", \"https://example.com/index\"]",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-19");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "unexpected registry allowed");
    assert_eq!(
        result.message,
        "`deny.toml` allows unexpected registries: https://example.com/index."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}

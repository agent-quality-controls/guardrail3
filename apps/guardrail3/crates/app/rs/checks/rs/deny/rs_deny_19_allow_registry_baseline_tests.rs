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

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-19"
            && result.severity == Severity::Error
            && result.title == "unexpected registry allowed"
            && result.message == "`deny.toml` allows unexpected registries: https://example.com/index."
            && result.file.as_deref() == Some("deny.toml")
    }));
}

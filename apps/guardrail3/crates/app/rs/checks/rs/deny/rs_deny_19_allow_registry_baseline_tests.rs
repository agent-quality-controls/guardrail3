use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn errors_on_unexpected_registry() {
    let deny = canonical_deny_toml_service().replace(
        "allow-registry = [\"https://github.com/rust-lang/crates.io-index\"]",
        "allow-registry = [\"https://github.com/rust-lang/crates.io-index\", \"https://example.com/index\"]",
    );
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-19"
            && result.severity == Severity::Error
            && result.title == "unexpected registry allowed"
            && result.message == "`deny.toml` allows unexpected registries: https://example.com/index."
    }));
}

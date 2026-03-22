use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn warns_when_tokio_full_is_not_banned() {
    let config = config_facts(&canonical_deny_toml_service().replace("deny = [\"full\"]", "deny = []"));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-21"
            && result.severity == Severity::Warn
            && result.title == "tokio full feature not banned"
            && result.file.as_deref() == Some("deny.toml")
    }));
}

#[test]
fn warns_when_tokio_allow_list_drifts() {
    let config = config_facts(&canonical_deny_toml_service().replace(
        "allow = [\"fs\", \"io-util\", \"macros\", \"net\", \"process\", \"rt-multi-thread\", \"signal\", \"sync\", \"time\"]",
        "allow = [\"rt-multi-thread\"]",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-21"
            && result.severity == Severity::Warn
            && result.title == "tokio allowed features changed"
            && result.message.contains("rt-multi-thread")
    }));
}

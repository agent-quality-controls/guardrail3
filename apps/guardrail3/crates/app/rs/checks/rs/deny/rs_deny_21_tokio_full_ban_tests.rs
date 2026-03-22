use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_when_tokio_full_is_not_banned() {
    let deny = canonical_deny_toml_service().replace("deny = [\"full\"]", "deny = []");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-21"
            && result.severity == Severity::Warn
            && result.title == "tokio full feature not banned"
    }));
}

#[test]
fn warns_when_tokio_allow_list_drifts() {
    let deny = canonical_deny_toml_service().replace(
        "allow = [\"rt-multi-thread\", \"macros\", \"net\", \"sync\", \"signal\", \"bytes\", \"default\", \"io-util\", \"time\"]",
        "allow = [\"rt-multi-thread\"]",
    );
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-21"
            && result.severity == Severity::Warn
            && result.title == "tokio allowed features changed"
    }));
}

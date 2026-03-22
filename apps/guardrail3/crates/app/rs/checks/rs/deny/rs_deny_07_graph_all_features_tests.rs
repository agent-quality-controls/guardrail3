use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn errors_when_all_features_missing() {
    let deny = canonical_deny_toml_service().replace("all-features = true\n", "");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-07"
            && result.severity == Severity::Error
            && result.title == "graph all-features must be true"
            && result.message == "`deny.toml` must set `[graph].all-features = true`."
    }));
}

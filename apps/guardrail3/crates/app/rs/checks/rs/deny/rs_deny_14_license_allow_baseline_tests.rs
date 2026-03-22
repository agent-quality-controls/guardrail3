use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn errors_when_license_baseline_is_missing() {
    let deny = canonical_deny_toml_service().replace("\"MIT\", ", "");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-14"
            && result.severity == Severity::Error
            && result.title == "baseline license missing"
            && result.message == "`deny.toml` is missing allowed license `MIT`."
    }));
}

use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn errors_when_advisories_baseline_is_weakened() {
    let deny = canonical_deny_toml_service().replace("yanked = \"warn\"", "yanked = \"allow\"");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-05"
            && result.severity == Severity::Error
            && result.title == "advisories `yanked` has wrong value"
            && result.message == "`deny.toml` must set `[advisories].yanked = \"warn\"`, found `allow`."
    }));
}

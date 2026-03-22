use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn errors_on_bans_allow_entries() {
    let deny = canonical_deny_toml_service().replace("skip = []", "skip = []\nallow = [\"lazy_static\"]");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-25"
            && result.severity == Severity::Error
            && result.title == "bans allow-list present"
            && result.message == "`deny.toml` has non-empty `[bans].allow`: lazy_static."
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-25"
            && result.severity == Severity::Error
            && result.title == "allow-list overrides deny-list"
            && result.message == "`deny.toml` allows `lazy_static` even though it is banned."
    }));
}

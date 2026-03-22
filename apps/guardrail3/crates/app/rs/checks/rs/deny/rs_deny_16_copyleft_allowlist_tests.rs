use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_on_copyleft_license() {
    let deny = canonical_deny_toml_service().replace(
        "\"MIT\", ",
        "\"MIT\", \"GPL-3.0\", ",
    );
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-16"
            && result.severity == Severity::Warn
            && result.title == "copyleft license allowed"
            && result.message == "`deny.toml` allows copyleft license `GPL-3.0`."
    }));
}

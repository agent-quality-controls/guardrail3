use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn errors_on_bans_allow_entries() {
    let config =
        config_facts(&canonical_deny_toml_service().replace("skip = []", "skip = []\nallow = [\"lazy_static\"]"));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-25"
            && result.severity == Severity::Error
            && result.title == "bans allow-list present"
            && result.message == "`deny.toml` has non-empty `[bans].allow`: lazy_static."
            && result.file.as_deref() == Some("deny.toml")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-25"
            && result.severity == Severity::Error
            && result.title == "allow-list overrides deny-list"
            && result.message == "`deny.toml` allows `lazy_static` even though it is banned."
            && result.file.as_deref() == Some("deny.toml")
    }));
}

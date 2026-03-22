use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};

#[test]
fn warns_and_inventories_allow_git_entries() {
    let deny = config_facts(&canonical_deny_toml_service().replace(
        "allow-git = []",
        "allow-git = [\"https://github.com/example/repo\"]",
    ));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-20"
            && result.severity == Severity::Warn
            && result.title == "allow-git is non-empty"
            && result.message == "`deny.toml` has non-empty `[sources].allow-git`."
            && result.file.as_deref() == Some("deny.toml")
            && !result.inventory
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-20"
            && result.severity == Severity::Info
            && result.title == "allow-git entry"
            && result.message == "`deny.toml` allows git source `https://github.com/example/repo`."
            && result.file.as_deref() == Some("deny.toml")
            && result.inventory
    }));
}

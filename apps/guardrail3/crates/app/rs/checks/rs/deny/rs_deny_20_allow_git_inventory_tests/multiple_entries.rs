use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_allow_git_sources,
};
use super::super::check;

#[test]
fn warns_once_and_inventories_each_allow_git_entry() {
    let config = config_facts(&set_allow_git_sources(
        &canonical_deny_toml_service(),
        &[
            "https://github.com/example/repo",
            "https://github.com/example/other",
        ],
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    let warnings = results
        .iter()
        .filter(|result| result.severity == Severity::Warn)
        .collect::<Vec<_>>();
    assert_eq!(
        warnings.len(),
        1,
        "expected one allow-git warning: {warnings:#?}"
    );
    assert_eq!(warnings[0].id, "RS-DENY-20");
    assert_eq!(warnings[0].title, "allow-git is non-empty");
    assert_eq!(
        warnings[0].message,
        "`deny.toml` has non-empty `[sources].allow-git`."
    );
    assert!(!warnings[0].inventory);

    let inventory_messages = results
        .iter()
        .filter(|result| result.inventory)
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_inventory_messages = BTreeSet::from([
        "`deny.toml` allows git source `https://github.com/example/other`.".to_owned(),
        "`deny.toml` allows git source `https://github.com/example/repo`.".to_owned(),
    ]);

    assert_eq!(inventory_messages, expected_inventory_messages);
    assert!(
        results
            .iter()
            .filter(|result| result.id == "RS-DENY-20")
            .all(|result| { result.file.as_deref() == Some("deny.toml") })
    );
}

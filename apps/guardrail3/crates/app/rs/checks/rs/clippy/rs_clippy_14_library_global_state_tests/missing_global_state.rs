use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, library_workspace_root_tree,
};
use super::super::check;

#[test]
fn errors_for_every_missing_library_global_state_type_ban() {
    let tree = library_workspace_root_tree(canonical_clippy_toml());
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(
        &config_input(&facts, "apps/libsite/clippy.toml"),
        &mut results,
    );

    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = BTreeSet::from([
        "Library profile must ban `once_cell::sync::Lazy` in `disallowed-types`.".to_owned(),
        "Library profile must ban `once_cell::sync::OnceCell` in `disallowed-types`.".to_owned(),
        "Library profile must ban `std::sync::LazyLock` in `disallowed-types`.".to_owned(),
        "Library profile must ban `std::sync::OnceLock` in `disallowed-types`.".to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);
    assert_eq!(results.len(), expected_messages.len());
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-14"
            && !result.inventory
            && result.severity == Severity::Error
            && result.title == "library clippy.toml missing global-state type ban"
            && result.file.as_deref() == Some("apps/libsite/clippy.toml")
    }));
}

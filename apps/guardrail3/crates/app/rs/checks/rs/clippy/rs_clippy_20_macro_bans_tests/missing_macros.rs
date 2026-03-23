use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, remove_ban_path, root_workspace_tree,
};
use super::super::check;

#[test]
fn errors_for_each_missing_required_macro_ban() {
    let clippy = remove_ban_path(
        &remove_ban_path(&canonical_clippy_toml(), "disallowed-macros", "eprintln"),
        "disallowed-macros",
        "todo",
    );
    let tree = root_workspace_tree(clippy);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    let error_messages = results
        .iter()
        .filter(|result| result.severity == Severity::Error)
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_error_messages = BTreeSet::from([
        "`eprintln!` is not present in `disallowed-macros`.".to_owned(),
        "`todo!` is not present in `disallowed-macros`.".to_owned(),
    ]);

    assert_eq!(error_messages, expected_error_messages);
    assert!(results.iter().all(|result| result.id == "RS-CLIPPY-20"));
}

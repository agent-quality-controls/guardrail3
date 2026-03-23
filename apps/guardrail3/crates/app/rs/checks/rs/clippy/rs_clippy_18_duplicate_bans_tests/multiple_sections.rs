use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::super::check;

#[test]
fn warns_once_per_duplicate_path_per_section() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = [
    { path = "std::env::var", reason = "good enough reason text" },
    { path = "std::env::var", reason = "another good enough reason text" },
]
disallowed-types = [
    { path = "std::collections::HashMap", reason = "good enough reason text" },
    { path = "std::collections::HashMap", reason = "another good enough reason text" },
]
disallowed-macros = [
    { path = "println", reason = "good enough reason text" },
    { path = "println", reason = "another good enough reason text" },
]
"#,
    );
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = BTreeSet::from([
        "`println` appears 2 times in `disallowed-macros`.".to_owned(),
        "`std::collections::HashMap` appears 2 times in `disallowed-types`.".to_owned(),
        "`std::env::var` appears 2 times in `disallowed-methods`.".to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-18"
            && result.severity == Severity::Warn
            && result.title == "duplicate ban entry"
            && !result.inventory
            && result.file.as_deref() == Some("clippy.toml")
    }));
}

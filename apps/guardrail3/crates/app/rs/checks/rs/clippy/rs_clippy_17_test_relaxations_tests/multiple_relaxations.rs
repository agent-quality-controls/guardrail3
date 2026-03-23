use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::super::check;

#[test]
fn warns_for_each_enabled_test_relaxation() {
    let tree = root_workspace_tree(
        r#"
allow-dbg-in-tests = true
allow-print-in-tests = true
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
        "`allow-dbg-in-tests = true` relaxes test output discipline.".to_owned(),
        "`allow-print-in-tests = true` relaxes test output discipline.".to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-17"
            && result.severity == Severity::Warn
            && result.title == "clippy test relaxation enabled"
            && !result.inventory
            && result.file.as_deref() == Some("clippy.toml")
    }));
}

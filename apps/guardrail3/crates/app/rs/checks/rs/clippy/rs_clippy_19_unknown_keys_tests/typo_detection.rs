use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::inputs::ConfigClippyInput;
use super::super::super::test_support::{collected_facts, root_workspace_tree};
use super::super::check;

#[test]
fn warns_for_managed_key_typos_but_not_unrelated_unknown_keys() {
    let tree = root_workspace_tree(
        "disalowed-methods = []\nallow-print-in-tets = false\ncustom-project-key = true",
    );
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();

    check(&input, &mut results);

    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = BTreeSet::from([
        "Top-level key `allow-print-in-tets` looks like a typo of a guardrail-managed clippy key."
            .to_owned(),
        "Top-level key `disalowed-methods` looks like a typo of a guardrail-managed clippy key."
            .to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);
    assert_eq!(results.len(), expected_messages.len());
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-19"
            && result.severity == Severity::Warn
            && result.title == "unrecognized clippy.toml key"
            && result.file.as_deref() == Some("clippy.toml")
    }));
}

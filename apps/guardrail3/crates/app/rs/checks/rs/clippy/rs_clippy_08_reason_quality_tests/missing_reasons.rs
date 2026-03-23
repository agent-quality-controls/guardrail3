use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::super::check;

#[test]
fn warns_for_plain_string_and_missing_reason_entries_across_sections() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = ["std::env::var"]
disallowed-types = [{ path = "std::collections::HashMap" }]
disallowed-macros = [{ path = "println", reason = "good enough reason text" }, "dbg"]
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
        "`dbg` in `disallowed-macros` must use table format with a `reason` field.".to_owned(),
        "`std::collections::HashMap` in `disallowed-types` must use table format with a `reason` field."
            .to_owned(),
        "`std::env::var` in `disallowed-methods` must use table format with a `reason` field."
            .to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-08"
            && !result.inventory
            && result.severity == Severity::Warn
            && result.title == "ban entry missing reason"
            && result.file.as_deref() == Some("clippy.toml")
    }));
}

use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::super::check;

#[test]
fn warns_for_placeholder_reasons_across_methods_types_and_macros() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = [
    { path = "std::env::var", reason = "todo" },
]
disallowed-types = [
    { path = "std::collections::HashMap", reason = "reason" },
]
disallowed-macros = [
    { path = "println", reason = "short" },
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
        "`println` in `disallowed-macros` has a trivial or placeholder `reason`.".to_owned(),
        "`std::collections::HashMap` in `disallowed-types` has a trivial or placeholder `reason`."
            .to_owned(),
        "`std::env::var` in `disallowed-methods` has a trivial or placeholder `reason`.".to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-15"
            && !result.inventory
            && result.severity == Severity::Warn
            && result.title == "ban entry has placeholder reason"
            && result.file.as_deref() == Some("clippy.toml")
    }));
}

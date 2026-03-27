use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, root_workspace_tree,
};
use super::super::check;

#[test]
fn inventories_every_required_macro_ban_from_generated_baseline() {
    let tree = root_workspace_tree(canonical_clippy_toml());
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = BTreeSet::from([
        "`dbg!` is banned.".to_owned(),
        "`eprintln!` is banned.".to_owned(),
        "`println!` is banned.".to_owned(),
        "`todo!` is banned.".to_owned(),
        "`unimplemented!` is banned.".to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-20"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "macro ban present"
            && result.file.as_deref() == Some("clippy.toml")
    }));
}

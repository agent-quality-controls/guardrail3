use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::clippy_support::{ban_paths, expected_method_bans};
use super::super::super::test_support::{
    build_fixture_clippy_toml, collected_facts, config_input, garde_disabled_root_tree,
};
use super::super::check;

#[test]
fn inventories_garde_owned_method_bans_as_project_specific_when_garde_is_disabled() {
    let tree = garde_disabled_root_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let facts = collected_facts(&tree);
    let input = config_input(&facts, "clippy.toml");
    let mut results = Vec::new();

    check(&input, &mut results);

    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_baseline = expected_method_bans(false);
    let expected_messages = ban_paths(
        input
            .config
            .parsed
            .as_ref()
            .expect("expected parsed clippy TOML"),
        "disallowed-methods",
    )
    .into_iter()
    .filter(|path| !expected_baseline.contains(&path.as_str()))
    .map(|path| format!("Additional method ban `{path}` beyond baseline."))
    .collect::<BTreeSet<_>>();

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-06"
            && result.inventory
            && result.severity == Severity::Info
            && result.file.as_deref() == Some("clippy.toml")
    }));
}

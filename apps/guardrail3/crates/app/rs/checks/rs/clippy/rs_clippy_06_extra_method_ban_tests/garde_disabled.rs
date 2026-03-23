use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, garde_disabled_root_tree,
};
use super::super::check;

#[test]
fn inventories_garde_owned_method_bans_as_project_specific_when_garde_is_disabled() {
    let tree = garde_disabled_root_tree(canonical_clippy_toml());
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = BTreeSet::from([
        "Additional method ban `reqwest::Response::json` beyond baseline.".to_owned(),
        "Additional method ban `serde_json::from_reader` beyond baseline.".to_owned(),
        "Additional method ban `serde_json::from_slice` beyond baseline.".to_owned(),
        "Additional method ban `serde_json::from_str` beyond baseline.".to_owned(),
        "Additional method ban `serde_json::from_value` beyond baseline.".to_owned(),
        "Additional method ban `serde_yaml::from_reader` beyond baseline.".to_owned(),
        "Additional method ban `serde_yaml::from_str` beyond baseline.".to_owned(),
        "Additional method ban `toml::from_str` beyond baseline.".to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-06"
            && result.inventory
            && result.severity == Severity::Info
            && result.file.as_deref() == Some("clippy.toml")
    }));
}

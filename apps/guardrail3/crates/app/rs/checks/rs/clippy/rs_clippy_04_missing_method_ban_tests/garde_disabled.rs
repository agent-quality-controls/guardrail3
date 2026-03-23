use crate::domain::report::Severity;

use super::super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, garde_disabled_root_tree, remove_ban_path,
};
use super::super::check;

#[test]
fn drops_garde_owned_method_requirements_when_garde_is_disabled() {
    let mut clippy = canonical_clippy_toml();
    for path in [
        "serde_json::from_str",
        "serde_json::from_slice",
        "serde_json::from_value",
        "serde_json::from_reader",
        "reqwest::Response::json",
        "toml::from_str",
        "serde_yaml::from_str",
        "serde_yaml::from_reader",
    ] {
        clippy = remove_ban_path(&clippy, "disallowed-methods", path);
    }

    let tree = garde_disabled_root_tree(clippy);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-04"
            && result.inventory
            && result.severity == Severity::Info
            && result.file.as_deref() == Some("clippy.toml")
    }));
    assert!(
        !results
            .iter()
            .any(|result| result.message.contains("serde_json::from_str"))
    );
    assert!(
        !results
            .iter()
            .any(|result| result.message.contains("reqwest::Response::json"))
    );
}

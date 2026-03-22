use crate::domain::report::Severity;

use super::super::inputs::ConfigClippyInput;
use super::super::test_support::{
    collected_facts, published_library_package_root_tree, root_workspace_tree,
};
use super::check;

#[test]
fn warns_when_avoid_breaking_exported_api_is_true_for_non_publishable_roots() {
    let tree = root_workspace_tree("avoid-breaking-exported-api = true");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-16"
            && result.severity == Severity::Warn
            && result.title == "avoid-breaking-exported-api enabled"
            && result.message
                == "`avoid-breaking-exported-api = true` suppresses useful lints. Prefer `false`."
    }));
}

#[test]
fn inventories_true_value_for_published_library_packages() {
    let tree = published_library_package_root_tree(
        "avoid-breaking-exported-api = true",
    );
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-16"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "library keeps avoid-breaking-exported-api enabled"
    }));
}

#[test]
fn warns_when_setting_is_missing() {
    let tree = root_workspace_tree("");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-16"
            && result.severity == Severity::Warn
            && result.title == "avoid-breaking-exported-api not set"
    }));
}

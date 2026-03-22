use crate::domain::report::Severity;

use super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, garde_disabled_root_tree,
    root_workspace_tree,
};
use super::check;

#[test]
fn inventories_baseline_method_bans() {
    let tree = root_workspace_tree(canonical_clippy_toml());
    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&config_input(&facts, "clippy.toml"), &mut results);
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-04" && result.inventory && matches!(result.severity, Severity::Info)
    }));
    assert!(results.iter().any(|result| {
        result.title == "method ban present" && result.message == "`std::env::var` is banned."
    }));
}

#[test]
fn skips_garde_method_bans_when_garde_disabled() {
    let mut clippy = canonical_clippy_toml().to_owned();
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
        clippy = clippy.replace(
            &format!("    {{ path = \"{path}\", reason = \"good enough reason text\" }},\n"),
            "",
        );
    }
    let tree = garde_disabled_root_tree(clippy);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&config_input(&facts, "clippy.toml"), &mut results);
    assert!(results.iter().all(|result| result.id == "RS-CLIPPY-04"));
    assert!(!results.iter().any(|r| r.id == "RS-CLIPPY-04" && r.message.contains("serde_json::from_str")));
}

#[test]
fn errors_when_required_method_ban_is_missing() {
    let clippy = canonical_clippy_toml().replace(
        "    { path = \"std::env::var\", reason = \"good enough reason text\" },\n",
        "",
    );
    let tree = root_workspace_tree(clippy);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&config_input(&facts, "clippy.toml"), &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-04"
            && matches!(result.severity, Severity::Error)
            && result.title == "missing method ban"
            && result.message == "`std::env::var` is not present in `disallowed-methods`."
    }));
}

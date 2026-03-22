use super::super::check;
use super::super::test_support::{canonical_clippy_toml, garde_disabled_root_tree, root_workspace_tree};

#[test]
fn inventories_baseline_method_bans() {
    let results = check(&root_workspace_tree(canonical_clippy_toml()));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-04" && r.inventory && r.message.contains("std::env::var")));
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
    let results = check(&garde_disabled_root_tree(clippy));
    assert!(!results.iter().any(|r| r.id == "RS-CLIPPY-04" && r.message.contains("serde_json::from_str")));
}

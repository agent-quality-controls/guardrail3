use crate::domain::report::Severity;

use super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, garde_disabled_root_tree,
    root_workspace_tree,
};
use super::check;

#[test]
fn inventories_baseline_type_bans() {
    let tree = root_workspace_tree(canonical_clippy_toml());
    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&config_input(&facts, "clippy.toml"), &mut results);
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-05" && result.inventory && matches!(result.severity, Severity::Info)
    }));
    assert!(results.iter().any(|result| {
        result.title == "type ban present"
            && result.message == "`std::collections::HashMap` is banned."
    }));
}

#[test]
fn skips_garde_type_bans_when_garde_disabled() {
    let mut clippy = canonical_clippy_toml().to_owned();
    for path in [
        "axum::extract::Json",
        "axum::Json",
        "axum::extract::Query",
        "axum::extract::Form",
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
    assert!(results.iter().all(|result| result.id == "RS-CLIPPY-05"));
    assert!(!results.iter().any(|r| r.id == "RS-CLIPPY-05" && r.message.contains("axum::extract::Json")));
}

#[test]
fn errors_when_required_type_ban_is_missing() {
    let clippy = canonical_clippy_toml().replace(
        "    { path = \"std::collections::HashMap\", reason = \"good enough reason text\" },\n",
        "",
    );
    let tree = root_workspace_tree(clippy);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&config_input(&facts, "clippy.toml"), &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-05"
            && matches!(result.severity, Severity::Error)
            && result.title == "missing type ban"
            && result.message
                == "`std::collections::HashMap` is not present in `disallowed-types`."
    }));
}

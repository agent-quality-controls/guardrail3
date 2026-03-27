use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    build_fixture_clippy_toml, collected_facts, config_input, garde_disabled_root_tree, remove_ban_path,
};
use super::super::check;

#[test]
fn drops_garde_owned_type_requirements_when_garde_is_disabled() {
    let mut clippy = build_fixture_clippy_toml("service", false, true, "", "");
    for path in [
        "axum::extract::Json",
        "axum::Json",
        "axum::extract::Query",
        "axum::extract::Form",
    ] {
        clippy = remove_ban_path(&clippy, "disallowed-types", path);
    }

    let tree = garde_disabled_root_tree(clippy);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-05"
            && result.inventory
            && result.severity == Severity::Info
            && result.file.as_deref() == Some("clippy.toml")
    }));
    assert!(
        !results
            .iter()
            .any(|result| result.message.contains("axum::extract::Json"))
    );
    assert!(
        !results
            .iter()
            .any(|result| result.message.contains("axum::extract::Form"))
    );
}

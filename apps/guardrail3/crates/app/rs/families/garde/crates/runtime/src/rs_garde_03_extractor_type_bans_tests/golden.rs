use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_domain_report::Severity;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn inventories_when_all_bans_present() {
    let root = temp_root("golden-garde-03");
    let clippy_toml = build_clippy_toml("service", false, true, "", "");
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "clippy.toml"]))],
        vec![
            (
                "Cargo.toml",
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n[dependencies]\ngarde = \"0.1\"\n",
            ),
            ("clippy.toml", clippy_toml.as_str()),
        ],
        root.clone(),
    );
    let results = crate::test_fixtures::run_family(&tree);
    let filtered: Vec<_> = results
        .into_iter()
        .filter(|r| r.id == "RS-GARDE-03")
        .collect();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].severity, Severity::Info);
    assert!(filtered[0].inventory);
    assert_eq!(
        filtered[0].message,
        "All required Axum extractor bans are present in the covering clippy configuration."
    );
    assert_eq!(filtered[0].file.as_deref(), Some("clippy.toml"));

    std::fs::remove_dir_all(&root).expect("remove temp root");
}

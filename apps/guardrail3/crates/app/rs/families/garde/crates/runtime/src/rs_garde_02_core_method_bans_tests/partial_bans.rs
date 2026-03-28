use crate::test_fixtures::{canonical_clippy_toml, remove_clippy_ban_path};
use guardrail3_domain_report::Severity;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn warns_when_bans_missing() {
    let root = temp_root("partial-garde-02");
    let mut clippy_toml = canonical_clippy_toml();
    for path in ["serde_json::from_slice", "serde_yaml::from_reader"] {
        clippy_toml = remove_clippy_ban_path(&clippy_toml, "disallowed-methods", path);
    }
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
        .filter(|r| r.id == "RS-GARDE-02")
        .collect();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].severity, Severity::Warn);
    assert_eq!(
        filtered[0].message,
        "Missing core garde deserialization bans from `disallowed-methods`: serde_json::from_slice, serde_yaml::from_reader."
    );
    assert_eq!(filtered[0].file.as_deref(), Some("clippy.toml"));

    std::fs::remove_dir_all(&root).expect("remove temp root");
}

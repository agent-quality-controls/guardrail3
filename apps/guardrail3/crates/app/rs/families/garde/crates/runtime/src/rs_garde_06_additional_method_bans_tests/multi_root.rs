use crate::test_fixtures::{canonical_clippy_toml, remove_clippy_ban_path};
use guardrail3_domain_report::Severity;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn local_missing_additional_ban_only_warns_for_owned_root() {
    let root = temp_root("multi-garde-06");
    let root_clippy = canonical_clippy_toml();
    let local_clippy = remove_clippy_ban_path(
        &canonical_clippy_toml(),
        "disallowed-methods",
        "serde_json::Deserializer::from_reader",
    );
    let tree = project_tree(
        vec![
            ("", dir_entry(&["vendor"], &["Cargo.toml", "clippy.toml"])),
            ("vendor", dir_entry(&["lib"], &[])),
            ("vendor/lib", dir_entry(&[], &["Cargo.toml", "clippy.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                "[workspace]\nmembers = []\n[workspace.dependencies]\ngarde = \"0.1\"\n",
            ),
            ("clippy.toml", root_clippy.as_str()),
            (
                "vendor/lib/Cargo.toml",
                "[package]\nname = \"vendored\"\nversion = \"0.1.0\"\n[dependencies]\ngarde = \"0.1\"\n",
            ),
            ("vendor/lib/clippy.toml", local_clippy.as_str()),
        ],
        root.clone(),
    );
    let results = crate::test_fixtures::run_family(&tree);
    let filtered: Vec<_> = results
        .into_iter()
        .filter(|r| r.id == "RS-GARDE-06")
        .collect();
    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().any(|result| {
        result.severity == Severity::Info
            && result.inventory
            && result.file.as_deref() == Some("clippy.toml")
    }));
    assert!(filtered.iter().any(|result| {
        result.severity == Severity::Warn
            && !result.inventory
            && result.file.as_deref() == Some("vendor/lib/clippy.toml")
            && result.message
                == "Missing additional garde deserialization bans from `disallowed-methods`: serde_json::Deserializer::from_reader."
    }));

    std::fs::remove_dir_all(&root).expect("remove temp root");
}

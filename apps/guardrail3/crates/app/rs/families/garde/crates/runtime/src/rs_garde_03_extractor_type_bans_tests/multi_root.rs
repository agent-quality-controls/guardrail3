use crate::test_fixtures::remove_clippy_ban_path;
use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_domain_report::Severity;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn local_missing_extractor_ban_only_warns_for_owned_root() {
    let root = temp_root("multi-garde-03");
    let root_clippy = build_clippy_toml("service", false, true, "", "");
    let local_clippy = remove_clippy_ban_path(
        &build_clippy_toml("service", false, true, "", ""),
        "disallowed-types",
        "axum::extract::Multipart",
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
        .filter(|r| r.id == "RS-GARDE-03")
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
                == "Missing garde extractor bans from `disallowed-types`: axum::extract::Multipart."
    }));

    std::fs::remove_dir_all(&root).expect("remove temp root");
}

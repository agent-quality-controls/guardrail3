use crate::test_support::{dir_entry, project_tree, remove_clippy_ban_path, temp_root};
use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_domain_report::Severity;

#[test]
fn warns_when_bans_missing() {
    let root = temp_root("partial-garde-03");
    let mut clippy_toml = build_clippy_toml("service", false, true, "", "");
    for path in ["axum::extract::Path", "axum_extra::extract::TypedHeader"] {
        clippy_toml = remove_clippy_ban_path(&clippy_toml, "disallowed-types", path);
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
    let results = crate::test_support::run_family(&tree);
    let filtered: Vec<_> = results
        .into_iter()
        .filter(|r| r.id == "RS-GARDE-03")
        .collect();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].severity, Severity::Warn);
    assert_eq!(
        filtered[0].message,
        "Missing garde extractor bans from `disallowed-types`: axum::extract::Path, axum_extra::extract::TypedHeader."
    );
    assert_eq!(filtered[0].file.as_deref(), Some("clippy.toml"));

    std::fs::remove_dir_all(&root).expect("remove temp root");
}

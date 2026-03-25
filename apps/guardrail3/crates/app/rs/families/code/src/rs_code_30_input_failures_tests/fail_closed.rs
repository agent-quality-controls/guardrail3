use std::collections::{BTreeMap, BTreeSet};

use guardrail3_domain_report::Severity;
use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

use super::super::super::test_support::{files_for_rule, temp_root};

#[test]
fn family_surfaces_source_parse_failures_with_exact_owned_hit_set() {
    let root = temp_root("rs-code-30-source-parse-failure");
    let source_rel = "src/lib.rs";
    let source_abs = root.join(source_rel);
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("create source dir");
    std::fs::write(&source_abs, "fn broken( {").expect("write source");

    let tree = ProjectTree {
        root: root.clone(),
        structure: BTreeMap::from([
            (
                String::new(),
                DirEntry {
                    dirs: vec!["src".to_owned()],
                    files: vec!["Cargo.toml".to_owned()],
                    symlink_dirs: vec![],
                    symlink_files: vec![],
                },
            ),
            (
                "src".to_owned(),
                DirEntry {
                    dirs: Vec::new(),
                    files: vec!["lib.rs".to_owned()],
                    symlink_dirs: vec![],
                    symlink_files: vec![],
                },
            ),
        ]),
        content: BTreeMap::from([(
            "Cargo.toml".to_owned(),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
        )]),
    };

    let results = crate::check(&tree, None);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-30"),
        BTreeSet::from([source_rel.to_owned()])
    );
    let result = results
        .iter()
        .find(|result| result.id == "RS-CODE-30" && result.file.as_deref() == Some(source_rel))
        .expect("source parse failure result");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "code-family input failure");
    assert_eq!(result.line, None);
    assert!(!result.inventory);
    assert!(result.message.contains("Failed to parse Rust source file"));

    std::fs::remove_dir_all(&root).expect("remove temp tree");
}

#[test]
fn family_surfaces_guardrail_policy_parse_failures_with_exact_owned_hit_set() {
    let root = temp_root("rs-code-30-guardrail-parse-failure");
    let source_rel = "src/lib.rs";
    let source_abs = root.join(source_rel);
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("create source dir");
    std::fs::write(
        &source_abs,
        "pub fn parse() -> Result<(), String> { Ok(()) }",
    )
    .expect("write source");

    let tree = ProjectTree {
        root: root.clone(),
        structure: BTreeMap::from([
            (
                String::new(),
                DirEntry {
                    dirs: vec!["src".to_owned()],
                    files: vec!["Cargo.toml".to_owned(), "guardrail3.toml".to_owned()],
                    symlink_dirs: vec![],
                    symlink_files: vec![],
                },
            ),
            (
                "src".to_owned(),
                DirEntry {
                    dirs: Vec::new(),
                    files: vec!["lib.rs".to_owned()],
                    symlink_dirs: vec![],
                    symlink_files: vec![],
                },
            ),
        ]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
            ),
            (
                "guardrail3.toml".to_owned(),
                "[rust.packages\n type = \"library\"\n".to_owned(),
            ),
        ]),
    };

    let results = crate::check(&tree, None);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-30"),
        BTreeSet::from(["guardrail3.toml".to_owned()])
    );
    let result = results
        .iter()
        .find(|result| {
            result.id == "RS-CODE-30" && result.file.as_deref() == Some("guardrail3.toml")
        })
        .expect("guardrail parse failure result");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "code-family input failure");
    assert_eq!(result.line, None);
    assert!(!result.inventory);
    assert!(result.message.contains("Failed to parse guardrail3.toml"));

    std::fs::remove_dir_all(&root).expect("remove temp tree");
}

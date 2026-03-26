use std::collections::BTreeSet;

use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

use super::super::super::test_support::{files_for_rule, temp_root};

#[test]
fn golden_tree_has_no_code_input_failures() {
    let root = temp_root("rs-code-30-golden");
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
        structure: std::collections::BTreeMap::from([
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
        content: std::collections::BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
            ),
            (
                "guardrail3.toml".to_owned(),
                "[rust.packages]\ntype = \"library\"\n".to_owned(),
            ),
        ]),
    };

    let results = crate::test_support::run_tree(&tree);
    let rs_code_30_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-30")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-30"), BTreeSet::new());
    assert!(rs_code_30_results.is_empty());

    std::fs::remove_dir_all(&root).expect("remove temp tree");
}

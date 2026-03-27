use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

use guardrail3_app_rs_family_code_assertions::rs_code_30_input_failures::{assert_no_hits};
use super::super::run_tree;
use test_support::temp_root;
use test_support::{create_dir_all, write_path};

#[test]
fn golden_tree_has_no_code_input_failures() {
    let root = temp_root("rs-code-30-golden");
    let source_rel = "src/lib.rs";
    let source_abs = root.join(source_rel);
    create_dir_all(source_abs.parent().unwrap_or(root.as_path()));
    write_path(&source_abs, "pub fn parse() -> Result<(), String> { Ok(()) }");

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

    let results = run_tree(&tree);
    assert_no_hits(&results);

}

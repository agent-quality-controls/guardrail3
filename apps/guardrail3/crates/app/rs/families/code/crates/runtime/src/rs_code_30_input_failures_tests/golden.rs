use super::super::{DirEntry, ProjectTree, run_tree};
use guardrail3_app_rs_family_code_assertions::rs_code_30_input_failures::assert_no_hits;
use test_support::{create_dir_all, create_temp_dir, write_path};

#[test]
fn golden_tree_has_no_code_input_failures() {
    let root = create_temp_dir("rs-code-30-golden");
    let source_rel = "src/lib.rs";
    let source_abs = root.path().join(source_rel);
    create_dir_all(source_abs.parent().unwrap_or(root.path()));
    write_path(
        &source_abs,
        "pub fn parse() -> Result<(), String> { Ok(()) }",
    );

    let tree = ProjectTree {
        root: root.path().to_path_buf(),
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

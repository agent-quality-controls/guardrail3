use super::helpers::{DirEntry, ProjectTree, run_tree};
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_30_input_failures::assert_no_hits;
use test_support::{create_dir_all, create_temp_dir, write_path};

fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|dir| (*dir).to_owned()).collect(),
        files.iter().map(|file| (*file).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}

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

    let tree = ProjectTree::build(
        root.path().to_path_buf(),
        &std::collections::BTreeMap::from([
            (
                String::new(),
                dir_entry(&["src"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("src".to_owned(), dir_entry(&[], &["lib.rs"])),
        ]),
        &std::collections::BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
            ),
            (
                "guardrail3.toml".to_owned(),
                "[rust.packages]\ntype = \"library\"\n".to_owned(),
            ),
        ]),
        &["".to_owned()],
        &[],
        &[],
        None,
        &[],
    );

    let results = run_tree(&tree);
    assert_no_hits(&results);
}

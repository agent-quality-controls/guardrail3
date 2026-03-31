use guardrail3_app_rs_family_clippy_assertions::rs_clippy_12_allowed_placement as assertions;
use test_support::{dir_entry, project_tree};

use super::super::run_for_tests;

#[test]
fn inventories_root_workspace_dotfile_only() {
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", ".clippy.toml"]))],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []\n".to_owned()),
            (".clippy.toml", "msrv = \"1.85\"\n".to_owned()),
        ],
    );

    let results = run_for_tests(&tree);
    assertions::assert_allowed_files(&results, &[".clippy.toml"]);
}

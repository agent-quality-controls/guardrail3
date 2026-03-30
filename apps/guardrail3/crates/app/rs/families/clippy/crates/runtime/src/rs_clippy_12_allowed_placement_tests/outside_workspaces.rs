use guardrail3_app_rs_family_clippy_assertions::rs_clippy_12_allowed_placement as assertions;
use test_support::dir_entry;

use super::super::run_for_tests;

#[test]
fn reports_stray_clippy_config_outside_all_workspaces() {
    let tree = test_support::project_tree(
        vec![
            ("", dir_entry(&["tools"], &[])),
            ("tools", dir_entry(&["helper"], &[])),
            ("tools/helper", dir_entry(&[], &["clippy.toml"])),
        ],
        vec![("tools/helper/clippy.toml", "msrv = \"1.85\"\n".to_owned())],
    );

    let results = run_for_tests(&tree);
    assertions::assert_forbidden_files(&results, &["tools/helper/clippy.toml"]);
}

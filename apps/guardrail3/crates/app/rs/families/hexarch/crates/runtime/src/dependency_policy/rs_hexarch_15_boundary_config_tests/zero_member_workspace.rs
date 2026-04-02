use guardrail3_app_rs_family_hexarch_assertions::dependency_policy::rs_hexarch_15_boundary_config as assertions;

use super::{dir_entry, project_tree};

#[test]
fn empty_workspace_app_still_requires_boundary_config() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&[], &[])),
        ],
        vec![
            ("guardrail3.toml", "[rust.checks]\nhexarch = true\n"),
            ("apps/api/Cargo.toml", "[workspace]\nmembers = []\n"),
        ],
    );

    let results = super::super::results_for_test_tree(&tree);
    assertions::assert_title_set(
        &results,
        "",
        1,
        &["app boundary `apps/api` missing rust.apps config"],
    );
}

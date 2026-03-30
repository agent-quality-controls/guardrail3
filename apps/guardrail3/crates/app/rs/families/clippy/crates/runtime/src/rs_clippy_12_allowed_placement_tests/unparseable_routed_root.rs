use guardrail3_app_rs_family_clippy_assertions::rs_clippy_12_allowed_placement as assertions;
use test_support::{dir_entry, project_tree};

use super::super::run_for_tests;

#[test]
fn errors_when_config_is_attached_to_malformed_routed_cargo_root() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&[], &["Cargo.toml", "clippy.toml"]),
            ),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/backend/clippy.toml",
                "max-struct-bools = 3\n".to_owned(),
            ),
        ],
    );

    let results = run_for_tests(&tree);
    assertions::assert_unparseable_cargo_root(
        &results,
        "apps/backend/clippy.toml",
        "apps/backend/Cargo.toml",
    );
}

use guardrail3_app_rs_family_clippy_assertions::rs_clippy_16_avoid_breaking_exported_api as assertions;
use test_support::{dir_entry, project_tree};

use super::helpers::run_for_tests;

#[test]
fn warns_when_library_package_is_explicitly_not_publishable() {
    let tree = project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "clippy.toml"]),
        )],
        vec![
            (
                "Cargo.toml",
                "[workspace]\nmembers = []\n[package]\nname = \"libcrate\"\npublish = false\n"
                    .to_owned(),
            ),
            (
                "guardrail3.toml",
                "[profile]\nname = \"library\"\n".to_owned(),
            ),
            (
                "clippy.toml",
                "avoid-breaking-exported-api = true".to_owned(),
            ),
        ],
    );

    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_warn_true(&results, "clippy.toml");
}

#[test]
fn warns_when_library_package_publish_shape_is_malformed() {
    let tree = project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "clippy.toml"]),
        )],
        vec![
            (
                "Cargo.toml",
                "[workspace]\nmembers = []\n[package]\nname = \"libcrate\"\npublish = 7\n"
                    .to_owned(),
            ),
            (
                "guardrail3.toml",
                "[profile]\nname = \"library\"\n".to_owned(),
            ),
            (
                "clippy.toml",
                "avoid-breaking-exported-api = true".to_owned(),
            ),
        ],
    );

    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_warn_true(&results, "clippy.toml");
}

#[test]
fn warns_when_library_workspace_has_no_publishable_members() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["apps"], &["guardrail3.toml", "clippy.toml"]),
            ),
            ("apps", dir_entry(&["libsite"], &[])),
            (
                "apps/libsite",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/libsite/crates", dir_entry(&["core"], &[])),
            ("apps/libsite/crates/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.apps.libsite]\ntype = \"library\"".to_owned(),
            ),
            (
                "apps/libsite/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/libsite/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\npublish = false\n".to_owned(),
            ),
            (
                "clippy.toml",
                "avoid-breaking-exported-api = false".to_owned(),
            ),
            (
                "apps/libsite/clippy.toml",
                "avoid-breaking-exported-api = true".to_owned(),
            ),
        ],
    );

    let results = run_for_tests(&tree, "apps/libsite/clippy.toml");
    assertions::assert_warn_true(&results, "apps/libsite/clippy.toml");
}

use test_support::{dir_entry, project_tree};

use super::super::run_for_tests;

#[test]
fn yields_no_result_when_policy_context_parseability_is_owned_by_rs_clippy_23() {
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
            ("guardrail3.toml", "[profile =".to_owned()),
            (
                "clippy.toml",
                test_support::build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/libsite/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/libsite/clippy.toml",
                test_support::build_fixture_clippy_toml("library", false, true, "", ""),
            ),
            (
                "apps/libsite/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
        ],
    );

    let results = run_for_tests(&tree, "apps/libsite/clippy.toml");
    assert!(
        results.is_empty(),
        "expected RS-CLIPPY-23 to own malformed policy context: {results:#?}"
    );
}

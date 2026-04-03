use guardrail3_app_rs_family_clippy_assertions::rs_clippy_24_forbid_clippy_conf_dir_override as assertions;
use test_support::{dir_entry, project_tree};

use super::helpers::run_for_tests;

#[test]
fn errors_when_cargo_config_override_surface_content_is_missing() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&[".cargo"], &["Cargo.toml", "clippy.toml"])),
            (".cargo", dir_entry(&[], &["config.toml"])),
        ],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            ("clippy.toml", "max-struct-bools = 3\n".to_owned()),
        ],
    );

    let results = run_for_tests(&tree);
    assertions::assert_parse_error(&results, ".cargo/config.toml");
}

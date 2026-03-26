use super::cases::{nested_hex_everywhere, owned_leaf_dirs};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_06_leaf_valid as assertions;
use super::{copy_fixture, write_file};

#[test]
fn gitkeep_only_leaf_is_valid_placeholder() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/placeholder/.gitkeep",
        "",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn inner_hex_owner_without_crates_becomes_leaf_error() {
    let tmp = copy_fixture();
    std::fs::remove_dir_all(
        tmp.path()
            .join("apps/backend/crates/adapters/inbound/mcp/crates"),
    )
    .expect("remove inner crates");

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/backend/crates/adapters/inbound/mcp"),
            file_contains: None,
            title_contains: Some(&["missing Cargo.toml"]),
            message_contains: None,
        }],
    );
}

#[test]
fn crate_leaf_with_gitkeep_is_valid_everywhere() {
    let tmp = copy_fixture();
    let leaf_dirs = owned_leaf_dirs(tmp.path(), "kept_crate");
    for rel in &leaf_dirs {
        write_file(tmp.path(), &format!("{rel}/.gitkeep"), "");
        write_file(
            tmp.path(),
            &format!("{rel}/Cargo.toml"),
            "[package]\nname = \"kept-crate\"\nversion = \"0.1.0\"\n",
        );
    }

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn nested_hex_with_gitkeep_placeholders_is_valid_everywhere() {
    let tmp = copy_fixture();
    let _leaf_dirs = nested_hex_everywhere(tmp.path(), "hex_keep");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

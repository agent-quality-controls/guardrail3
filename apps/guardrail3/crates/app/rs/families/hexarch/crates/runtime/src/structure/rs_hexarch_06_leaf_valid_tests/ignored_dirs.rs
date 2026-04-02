use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::structure::rs_hexarch_06_leaf_valid as assertions;

#[test]
fn ignored_untracked_invalid_leaf_still_errors() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "orphan/\n");
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/orphan/src/lib.rs",
        "pub fn orphan() {}\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/crates/app/orphan"),
            file_contains: None,
            title_contains: Some(&["missing Cargo.toml"]),
            message_contains: None,
        }],
    );
}

#[test]
fn ignored_untracked_valid_crate_leaf_stays_valid() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "valid_crate/\n");
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/valid_crate/Cargo.toml",
        "[package]\nname = \"valid-crate\"\nversion = \"0.1.0\"\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn ignored_untracked_valid_nested_hex_leaf_stays_valid() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "valid_hex/\n");
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/valid_hex/crates/app/.gitkeep",
        "",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn ignored_untracked_gitkeep_placeholder_leaf_stays_valid() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "future_leaf/\n");
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/future_leaf/.gitkeep",
        "",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn ignored_untracked_hybrid_leaf_still_hits_both_branch() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "hybrid_leaf/\n");
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/hybrid_leaf/Cargo.toml",
        "[package]\nname = \"hybrid-leaf\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/hybrid_leaf/crates/app/.gitkeep",
        "",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/crates/app/hybrid_leaf"),
            file_contains: None,
            title_contains: Some(&["both Cargo.toml and crates/"]),
            message_contains: None,
        }],
    );
}

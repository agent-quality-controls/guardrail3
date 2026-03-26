use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_09_no_extra_workspace_members as assertions;
use test_support::{copy_fixture, write_file};

#[test]
fn normalized_and_glob_internal_members_are_not_extra_members() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "./crates/domain/types/",
    "crates/app/core",
    "crates/ports/outbound/traits",
    "crates/adapters/inbound/cli",
    "crates/adapters/outbound/fs",
]
resolver = "2"
"#,
    );

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-09").is_empty(),
        "{results:#?}"
    );
}

#[test]
fn crates_root_member_is_extra_not_outside_boundary() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "crates/domain/types",
    "crates/app/core",
    "crates/ports/outbound/traits",
    "crates/adapters/inbound/cli",
    "crates/adapters/outbound/fs",
    "./crates/",
]
resolver = "2"
"#,
    );

    let results = assertions::run_family(tmp.path());
    let rule_09 = assertions::errors_by_id(&results, "RS-HEXARCH-09");
    let rule_10 = assertions::errors_by_id(&results, "RS-HEXARCH-10");

    assert_eq!(
        rule_09.len(),
        1,
        "rule 09 should own invalid in-boundary roots: {rule_09:#?}"
    );
    assert!(
        rule_10.is_empty(),
        "rule 10 should not misclassify in-boundary `crates` members: {rule_10:#?}"
    );
    assert!(rule_09[0].title.contains("./crates/"));
}

#[test]
fn leave_and_reenter_same_app_is_not_extra_member() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "./../devctl/crates/domain/types/",
    "crates/app/core",
    "crates/ports/outbound/traits",
    "crates/adapters/inbound/cli",
    "crates/adapters/outbound/fs",
]
resolver = "2"
"#,
    );

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-09").is_empty(),
        "{results:#?}"
    );
}

#[test]
fn broad_glob_matching_container_dirs_is_still_an_extra_member() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "crates/*/*",
    "crates/ports/outbound/traits",
    "crates/adapters/inbound/cli",
    "crates/adapters/outbound/fs",
]
resolver = "2"
"#,
    );

    let results = assertions::run_family(tmp.path());
    let rule_09 = assertions::errors_by_id(&results, "RS-HEXARCH-09");
    let rule_10 = assertions::errors_by_id(&results, "RS-HEXARCH-10");

    assert_eq!(
        rule_09.len(),
        1,
        "expected broad container-matching glob to stay owned by rule 09: {rule_09:#?}"
    );
    assert!(
        rule_10.is_empty(),
        "rule 10 should not own in-boundary broad globs: {rule_10:#?}"
    );
    assert!(rule_09[0].title.contains("crates/*/*"));
}

use super::super::super::test_support::{
    assert_no_error, copy_fixture, errors_by_id, run_family, write_file,
};

#[test]
fn normalized_internal_members_do_not_trigger_rule_10() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "./crates/domain/types/",
    "crates/app/core",
    "crates/ports/outbound/*",
    "crates/adapters/inbound/cli",
    "crates/adapters/outbound/fs",
]
resolver = "2"
"#,
    );

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-10");
}

#[test]
fn normalized_outside_boundary_member_still_hits_rule_10() {
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
    "./../../packages/shared-types/",
]
resolver = "2"
"#,
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-10");

    assert_eq!(
        errors.len(),
        1,
        "expected one outside-boundary error: {errors:#?}"
    );
    assert!(errors[0].title.contains("./../../packages/shared-types/"));
}

#[test]
fn parent_escape_into_top_level_crates_still_hits_rule_10() {
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
    "../../packages/shared-types",
]
resolver = "2"
"#,
    );

    let results = run_family(tmp.path());
    let rule_10 = errors_by_id(&results, "RS-HEXARCH-10");
    assert_eq!(
        rule_10.len(),
        1,
        "expected one outside-boundary parent-escape error: {rule_10:#?}"
    );
    assert!(rule_10[0].title.contains("../../packages/shared-types"));
}

#[test]
fn parent_escape_glob_still_hits_rule_10() {
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
    "../crates/*",
]
resolver = "2"
"#,
    );

    let results_2 = run_family(tmp.path());
    let rule_10 = errors_by_id(&results_2, "RS-HEXARCH-10");
    assert_eq!(
        rule_10.len(),
        1,
        "expected one outside-boundary glob error: {rule_10:#?}"
    );
    assert!(rule_10[0].title.contains("../crates/*"));
}

#[test]
fn leave_and_reenter_same_app_does_not_false_positive_under_rule_10() {
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

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-10");
}

#[test]
fn absolute_member_path_stays_owned_by_rule_10() {
    let tmp = copy_fixture();
    let absolute_member = tmp
        .path()
        .join("apps/devctl/crates/domain/types")
        .display()
        .to_string();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        &format!(
            r#"[workspace]
members = [
    "crates/domain/types",
    "crates/app/core",
    "crates/ports/outbound/traits",
    "crates/adapters/inbound/cli",
    "crates/adapters/outbound/fs",
    "{absolute_member}",
]
resolver = "2"
"#
        ),
    );

    let results = run_family(tmp.path());
    let rule_07 = errors_by_id(&results, "RS-HEXARCH-07");
    let rule_09 = errors_by_id(&results, "RS-HEXARCH-09");
    let rule_10 = errors_by_id(&results, "RS-HEXARCH-10");

    assert!(
        rule_07.is_empty(),
        "rule 07 should not treat absolute members as valid internal coverage: {rule_07:#?}"
    );
    assert!(
        rule_09.is_empty(),
        "rule 09 should not own absolute members: {rule_09:#?}"
    );
    assert_eq!(
        rule_10.len(),
        1,
        "rule 10 should own absolute members: {rule_10:#?}"
    );
    assert!(rule_10[0].title.contains(&absolute_member));
}

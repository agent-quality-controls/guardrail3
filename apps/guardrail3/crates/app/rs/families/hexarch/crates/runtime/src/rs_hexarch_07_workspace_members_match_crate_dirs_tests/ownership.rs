use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_07_workspace_members_match_crate_dirs as assertions;
use test_support::{copy_fixture, write_file};

#[test]
fn malformed_app_cargo_is_owned_by_rule_08_not_rule_07() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/Cargo.toml", "[workspace");

    let results = assertions::run_family(tmp.path());
    let rule_07 = assertions::errors_by_id(&results, "RS-HEXARCH-07");
    let rule_08 = assertions::errors_by_id(&results, "RS-HEXARCH-08");
    assert!(
        rule_07.is_empty(),
        "rule 07 should skip parse-broken app cargo: {rule_07:#?}"
    );
    assert_eq!(
        rule_08.len(),
        1,
        "rule 08 should own parse errors: {rule_08:#?}"
    );
}

#[test]
fn package_style_app_cargo_is_owned_by_rule_08_not_rule_07() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[package]\nname = \"devctl\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/events/Cargo.toml",
        "[package]\nname = \"devctl-domain-events\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/events/src/lib.rs",
        "// events",
    );

    let results = assertions::run_family(tmp.path());
    let rule_07 = assertions::errors_by_id(&results, "RS-HEXARCH-07");
    let rule_08 = assertions::errors_by_id(&results, "RS-HEXARCH-08");
    assert!(
        rule_07.is_empty(),
        "rule 07 should not double-fire on non-workspace app cargo: {rule_07:#?}"
    );
    assert_eq!(
        rule_08.len(),
        1,
        "rule 08 should own non-workspace apps: {rule_08:#?}"
    );
}

#[test]
fn phantom_workspace_member_stays_owned_by_rule_09_not_rule_07() {
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
    "crates/domain/phantom",
]
resolver = "2"
"#,
    );

    let results = assertions::run_family(tmp.path());
    let rule_07 = assertions::errors_by_id(&results, "RS-HEXARCH-07");
    let rule_09 = assertions::errors_by_id(&results, "RS-HEXARCH-09");
    assert!(
        rule_07.is_empty(),
        "rule 07 should not own phantom members: {rule_07:#?}"
    );
    assert_eq!(
        rule_09.len(),
        1,
        "rule 09 should own phantom members: {rule_09:#?}"
    );
}

#[test]
fn outside_boundary_workspace_member_stays_owned_by_rule_10_not_rule_07() {
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

    let results = assertions::run_family(tmp.path());
    let rule_07 = assertions::errors_by_id(&results, "RS-HEXARCH-07");
    let rule_10 = assertions::errors_by_id(&results, "RS-HEXARCH-10");
    assert!(
        rule_07.is_empty(),
        "rule 07 should not own out-of-boundary members: {rule_07:#?}"
    );
    assert_eq!(
        rule_10.len(),
        1,
        "rule 10 should own out-of-boundary members: {rule_10:#?}"
    );
}

#[test]
fn mixed_missing_phantom_and_outside_members_split_cleanly_across_rules() {
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
    "crates/domain/phantom",
    "../../packages/shared-types",
]
resolver = "2"
"#,
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/events/Cargo.toml",
        "[package]\nname = \"devctl-domain-events\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/events/src/lib.rs",
        "// events",
    );

    let results = assertions::run_family(tmp.path());
    let rule_07 = assertions::errors_by_id(&results, "RS-HEXARCH-07");
    let rule_09 = assertions::errors_by_id(&results, "RS-HEXARCH-09");
    let rule_10 = assertions::errors_by_id(&results, "RS-HEXARCH-10");

    assert_eq!(
        rule_07.len(),
        1,
        "rule 07 should own only the missing crate: {rule_07:#?}"
    );
    assert_eq!(
        rule_09.len(),
        1,
        "rule 09 should own only the phantom member: {rule_09:#?}"
    );
    assert_eq!(
        rule_10.len(),
        1,
        "rule 10 should own only the out-of-boundary member: {rule_10:#?}"
    );
    assert!(rule_07[0].title.contains("crates/domain/events"));
    assert!(rule_09[0].title.contains("crates/domain/phantom"));
    assert!(rule_10[0].title.contains("../../packages/shared-types"));
}

#[test]
fn normalized_and_glob_internal_members_do_not_false_positive_under_rules_09_or_10() {
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
    let rule_07 = assertions::errors_by_id(&results, "RS-HEXARCH-07");
    let rule_09 = assertions::errors_by_id(&results, "RS-HEXARCH-09");
    let rule_10 = assertions::errors_by_id(&results, "RS-HEXARCH-10");

    assert!(
        rule_07.is_empty(),
        "rule 07 should accept valid internal member semantics: {rule_07:#?}"
    );
    assert!(
        rule_09.is_empty(),
        "rule 09 should not reject normalized/glob internal members: {rule_09:#?}"
    );
    assert!(
        rule_10.is_empty(),
        "rule 10 should not reject normalized internal members: {rule_10:#?}"
    );
}

#[test]
fn non_string_workspace_member_is_owned_by_rule_08_not_workspace_coverage_rules() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "crates/domain/types",
    42,
]
resolver = "2"
"#,
    );

    let results = assertions::run_family(tmp.path());
    let rule_07 = assertions::errors_by_id(&results, "RS-HEXARCH-07");
    let rule_08 = assertions::errors_by_id(&results, "RS-HEXARCH-08");
    let rule_09 = assertions::errors_by_id(&results, "RS-HEXARCH-09");
    let rule_10 = assertions::errors_by_id(&results, "RS-HEXARCH-10");

    assert!(
        rule_07.is_empty(),
        "rule 07 should skip semantically invalid workspace members: {rule_07:#?}"
    );
    assert_eq!(
        rule_08.len(),
        1,
        "rule 08 should own invalid workspace members: {rule_08:#?}"
    );
    assert!(
        rule_09.is_empty(),
        "rule 09 should skip semantically invalid workspace members: {rule_09:#?}"
    );
    assert!(
        rule_10.is_empty(),
        "rule 10 should skip semantically invalid workspace members: {rule_10:#?}"
    );
}

#[test]
fn same_app_reentry_glob_does_not_false_positive_under_workspace_coverage_rules() {
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
    let rule_07 = assertions::errors_by_id(&results, "RS-HEXARCH-07");
    let rule_09 = assertions::errors_by_id(&results, "RS-HEXARCH-09");
    let rule_10 = assertions::errors_by_id(&results, "RS-HEXARCH-10");

    assert!(
        rule_07.is_empty(),
        "rule 07 should accept same-app reentry globs: {rule_07:#?}"
    );
    assert!(
        rule_09.is_empty(),
        "rule 09 should not reject same-app reentry globs: {rule_09:#?}"
    );
    assert!(
        rule_10.is_empty(),
        "rule 10 should not reject same-app reentry globs: {rule_10:#?}"
    );
}

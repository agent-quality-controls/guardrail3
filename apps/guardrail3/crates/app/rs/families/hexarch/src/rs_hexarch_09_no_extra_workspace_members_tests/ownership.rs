use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn package_style_app_cargo_is_owned_by_rule_08_not_rule_09() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[package]\nname = \"devctl\"\nversion = \"0.1.0\"\n",
    );

    let results = run_family(tmp.path());
    let rule_08 = errors_by_id(&results, "RS-HEXARCH-08");
    let rule_09 = errors_by_id(&results, "RS-HEXARCH-09");

    assert!(
        rule_09.is_empty(),
        "rule 09 should not double-fire on non-workspace app cargo: {rule_09:#?}"
    );
    assert_eq!(
        rule_08.len(),
        1,
        "rule 08 should own non-workspace app cargo: {rule_08:#?}"
    );
}

#[test]
fn outside_boundary_workspace_member_stays_owned_by_rule_10_not_rule_09() {
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
    let rule_09 = errors_by_id(&results, "RS-HEXARCH-09");
    let rule_10 = errors_by_id(&results, "RS-HEXARCH-10");

    assert!(
        rule_09.is_empty(),
        "rule 09 should not own outside-boundary members: {rule_09:#?}"
    );
    assert_eq!(
        rule_10.len(),
        1,
        "rule 10 should own outside-boundary members: {rule_10:#?}"
    );
}

#[test]
fn parent_escape_into_top_level_crates_stays_owned_by_rule_10_not_rule_09() {
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
    let rule_09 = errors_by_id(&results, "RS-HEXARCH-09");
    let rule_10 = errors_by_id(&results, "RS-HEXARCH-10");

    assert!(
        rule_09.is_empty(),
        "rule 09 should not own parent-escape members: {rule_09:#?}"
    );
    assert_eq!(
        rule_10.len(),
        1,
        "rule 10 should own parent-escape members: {rule_10:#?}"
    );
}

#[test]
fn absolute_workspace_member_stays_owned_by_rule_10_not_rule_09() {
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
    "/crates/domain/types",
]
resolver = "2"
"#,
    );

    let results = run_family(tmp.path());
    let rule_09 = errors_by_id(&results, "RS-HEXARCH-09");
    let rule_10 = errors_by_id(&results, "RS-HEXARCH-10");

    assert!(
        rule_09.is_empty(),
        "rule 09 should not own absolute members: {rule_09:#?}"
    );
    assert_eq!(
        rule_10.len(),
        1,
        "rule 10 should own absolute members: {rule_10:#?}"
    );
}

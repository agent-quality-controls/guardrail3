use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_10_members_within_app_boundary as assertions;
use test_support::{copy_fixture, write_file};

#[test]
fn package_style_app_cargo_is_owned_by_rule_08_not_rule_10() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[package]\nname = \"devctl\"\nversion = \"0.1.0\"\n",
    );

    let results = assertions::run_family(tmp.path());
    let rule_08 = assertions::errors_by_id(&results, "RS-HEXARCH-08");
    let rule_10 = assertions::errors_by_id(&results, "RS-HEXARCH-10");

    assert!(
        rule_10.is_empty(),
        "rule 10 should not double-fire on non-workspace app cargo: {rule_10:#?}"
    );
    assert_eq!(
        rule_08.len(),
        1,
        "rule 08 should own non-workspace app cargo: {rule_08:#?}"
    );
}

#[test]
fn phantom_workspace_member_stays_owned_by_rule_09_not_rule_10() {
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
    let rule_09 = assertions::errors_by_id(&results, "RS-HEXARCH-09");
    let rule_10 = assertions::errors_by_id(&results, "RS-HEXARCH-10");

    assert_eq!(
        rule_09.len(),
        1,
        "rule 09 should own phantom members: {rule_09:#?}"
    );
    assert!(
        rule_10.is_empty(),
        "rule 10 should not misclassify in-boundary phantom members: {rule_10:#?}"
    );
}

#[test]
fn crates_root_member_stays_owned_by_rule_09_not_rule_10() {
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
}

#[test]
fn app_root_members_are_owned_by_rule_10_not_rules_07_or_09() {
    for member in [".", "./", ""] {
        let tmp = copy_fixture();
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
    "{member}",
]
resolver = "2"
"#
            ),
        );

        let results = assertions::run_family(tmp.path());
        let rule_07 = assertions::errors_by_id(&results, "RS-HEXARCH-07");
        let rule_09 = assertions::errors_by_id(&results, "RS-HEXARCH-09");
        let rule_10 = assertions::errors_by_id(&results, "RS-HEXARCH-10");

        assert!(
            rule_07.is_empty(),
            "rule 07 should not own app-root members `{member}`: {rule_07:#?}"
        );
        assert!(
            rule_09.is_empty(),
            "rule 09 should not own app-root members `{member}`: {rule_09:#?}"
        );
        assert_eq!(
            rule_10.len(),
            1,
            "rule 10 should own app-root members `{member}`: {rule_10:#?}"
        );
        if !member.is_empty() {
            assert!(rule_10[0].title.contains(member));
        }
    }
}

#[test]
fn absolute_workspace_member_stays_owned_by_rule_10_not_rule_08() {
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

    let results = assertions::run_family(tmp.path());
    let rule_08 = assertions::errors_by_id(&results, "RS-HEXARCH-08");
    let rule_10 = assertions::errors_by_id(&results, "RS-HEXARCH-10");

    assert!(
        rule_08.is_empty(),
        "rule 08 should not own absolute workspace members: {rule_08:#?}"
    );
    assert_eq!(
        rule_10.len(),
        1,
        "rule 10 should own absolute workspace members: {rule_10:#?}"
    );
    assert!(rule_10[0].title.contains(&absolute_member));
}

#[test]
fn sibling_app_member_stays_owned_by_rule_10_not_rules_07_or_09() {
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
    "../backend/crates/domain/engine",
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
        "rule 07 should not own sibling-app members: {rule_07:#?}"
    );
    assert!(
        rule_09.is_empty(),
        "rule 09 should not own sibling-app members: {rule_09:#?}"
    );
    assert_eq!(
        rule_10.len(),
        1,
        "rule 10 should own sibling-app members: {rule_10:#?}"
    );
    assert!(rule_10[0].title.contains("../backend/crates/domain/engine"));
}

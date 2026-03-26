use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_07_workspace_members_match_crate_dirs as assertions;
use super::{copy_fixture, write_file};

#[test]
fn malformed_app_cargo_is_owned_by_rule_08_not_rule_07() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/Cargo.toml", "[workspace");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
    assertions::assert_error_count(&results, "RS-HEXARCH-08", 1);
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
    assertions::assert_error_count(&results, "RS-HEXARCH-08", 1);
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
    assertions::assert_error_count(&results, "RS-HEXARCH-09", 1);
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
    assertions::assert_error_count(&results, "RS-HEXARCH-10", 1);
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

    let results = super::run_family(tmp.path());
    assertions::assert_error_count(&results, "", 1);
    assertions::assert_error_count(&results, "RS-HEXARCH-09", 1);
    assertions::assert_error_count(&results, "RS-HEXARCH-10", 1);
    assertions::assert_expected_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                file: Some("apps/devctl"),
                file_contains: None,
                title_contains: Some(&["crates/domain/events"]),
                message_contains: None,
            },
        ],
    );
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
    assertions::assert_no_error(&results, "RS-HEXARCH-09");
    assertions::assert_no_error(&results, "RS-HEXARCH-10");
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
    assertions::assert_error_count(&results, "RS-HEXARCH-08", 1);
    assertions::assert_no_error(&results, "RS-HEXARCH-09");
    assertions::assert_no_error(&results, "RS-HEXARCH-10");
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
    assertions::assert_no_error(&results, "RS-HEXARCH-09");
    assertions::assert_no_error(&results, "RS-HEXARCH-10");
}

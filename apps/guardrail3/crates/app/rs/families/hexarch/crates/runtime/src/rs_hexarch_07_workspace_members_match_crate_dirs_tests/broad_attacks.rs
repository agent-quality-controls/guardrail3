use std::collections::BTreeSet;

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_07_workspace_members_match_crate_dirs as assertions;
use super::{copy_fixture, write_file};

#[test]
fn discovered_crates_missing_from_workspace_members_hit_every_mutated_app() {
    let tmp = copy_fixture();
    for (rel, name) in [
        ("apps/devctl/crates/domain/events", "devctl-domain-events"),
        ("apps/backend/crates/domain/events", "backend-domain-events"),
        ("apps/worker/crates/domain/events", "worker-domain-events"),
    ] {
        write_file(
            tmp.path(),
            &format!("{rel}/Cargo.toml"),
            &format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\n"),
        );
        write_file(tmp.path(), &format!("{rel}/src/lib.rs"), "// events");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                file: Some("apps/devctl"),
                file_contains: None,
                title_contains: Some(&["not a workspace member", "crates/domain/events"]),
                message_contains: None,
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/backend"),
                file_contains: None,
                title_contains: Some(&["not a workspace member", "crates/domain/events"]),
                message_contains: None,
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/worker"),
                file_contains: None,
                title_contains: Some(&["not a workspace member", "crates/domain/events"]),
                message_contains: None,
            },
        ],
    );
}

#[test]
fn one_app_with_two_missing_crates_emits_two_errors() {
    let tmp = copy_fixture();
    for (rel, name) in [
        ("apps/devctl/crates/domain/events", "devctl-domain-events"),
        ("apps/devctl/crates/app/service", "devctl-app-service"),
    ] {
        write_file(
            tmp.path(),
            &format!("{rel}/Cargo.toml"),
            &format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\n"),
        );
        write_file(tmp.path(), &format!("{rel}/src/lib.rs"), "// new crate");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                file: Some("apps/devctl"),
                file_contains: None,
                title_contains: Some(&["crates/domain/events"]),
                message_contains: None,
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/devctl"),
                file_contains: None,
                title_contains: Some(&["crates/app/service"]),
                message_contains: None,
            },
        ],
    );
}

#[test]
fn one_app_with_one_missing_top_level_crate_emits_one_owned_error() {
    let tmp = copy_fixture();
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
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl"),
            file_contains: None,
            title_contains: Some(&["crates/domain/events"]),
            message_contains: None,
        }],
    );
}

#[test]
fn nested_inner_hex_missing_member_is_owned_by_backend_app_workspace() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/ports/outbound/events/Cargo.toml",
        "[package]\nname = \"backend-mcp-ports-outbound-events\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/ports/outbound/events/src/lib.rs",
        "// nested events",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/backend"),
            file_contains: None,
            title_contains: Some(&["crates/adapters/inbound/mcp/crates/ports/outbound/events"]),
            message_contains: None,
        }],
    );
}

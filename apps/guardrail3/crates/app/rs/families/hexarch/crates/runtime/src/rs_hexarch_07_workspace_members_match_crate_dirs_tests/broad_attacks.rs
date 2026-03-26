use std::collections::BTreeSet;

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_07_workspace_members_match_crate_dirs as assertions;
use crate::test_support::{copy_fixture, write_file};

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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-07");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/devctl", "apps/backend", "apps/worker"]
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("not a workspace member"));
        assert!(error.title.contains("crates/domain/events"));
    }
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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-07");

    assert_eq!(
        errors.len(),
        2,
        "expected one error per missing crate: {errors:#?}"
    );
    assert!(
        errors
            .iter()
            .all(|error| error.file.as_deref() == Some("apps/devctl")),
        "unexpected ownership: {errors:#?}"
    );
    let titles = errors
        .iter()
        .map(|error| error.title.as_str())
        .collect::<Vec<_>>();
    assert!(
        titles
            .iter()
            .any(|title| title.contains("crates/domain/events")),
        "missing domain/events attribution: {errors:#?}"
    );
    assert!(
        titles
            .iter()
            .any(|title| title.contains("crates/app/service")),
        "missing app/service attribution: {errors:#?}"
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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-07");

    assert_eq!(
        errors.len(),
        1,
        "expected exactly one missing top-level crate hit: {errors:#?}"
    );
    assert_eq!(
        errors[0].file.as_deref(),
        Some("apps/devctl"),
        "expected devctl ownership only: {errors:#?}"
    );
    assert!(
        errors[0].title.contains("crates/domain/events"),
        "missing exact crate attribution: {errors:#?}"
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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-07");

    assert_eq!(
        errors.len(),
        1,
        "expected one backend workspace error: {errors:#?}"
    );
    assert_eq!(errors[0].file.as_deref(), Some("apps/backend"));
    assert!(
        errors[0]
            .title
            .contains("crates/adapters/inbound/mcp/crates/ports/outbound/events")
    );
}

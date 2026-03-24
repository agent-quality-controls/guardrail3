use super::super::super::test_support::{assert_no_error, copy_fixture, run_family, write_file};

#[test]
fn normalized_member_path_counts_as_covered() {
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
    "./crates/domain/../domain/events/",
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

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-07");
}

#[test]
fn glob_member_pattern_counts_as_covered() {
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
    "./crates/domain/*/",
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

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-07");
}

#[test]
fn normalized_nested_inner_member_path_counts_as_covered() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        r#"[workspace]
members = [
    "crates/domain/types",
    "crates/domain/engine",
    "crates/app/commands",
    "crates/app/queries",
    "crates/ports/inbound/api",
    "crates/ports/outbound/repo",
    "crates/ports/outbound/events",
    "crates/adapters/inbound/rest",
    "crates/adapters/inbound/mcp/crates/domain/protocol",
    "crates/adapters/inbound/mcp/crates/app/handlers",
    "crates/adapters/inbound/mcp/crates/adapters/inbound/transport",
    "crates/adapters/outbound/postgres",
    "crates/adapters/outbound/queue",
    "./crates/adapters/inbound/mcp/crates/ports/outbound/../outbound/events/",
]
resolver = "2"
"#,
    );
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

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-07");
}

#[test]
fn leave_and_reenter_same_app_member_counts_as_covered() {
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
    assert_no_error(&results, "RS-HEXARCH-07");
}

#[test]
fn parent_escape_member_does_not_cover_internal_crate() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "../crates/domain/types",
    "crates/app/core",
    "crates/ports/outbound/traits",
    "crates/adapters/inbound/cli",
    "crates/adapters/outbound/fs",
]
resolver = "2"
"#,
    );

    let results = run_family(tmp.path());
    let errors = super::super::super::test_support::errors_by_id(&results, "RS-HEXARCH-07");
    assert_eq!(
        errors.len(),
        1,
        "expected one missing internal crate error: {errors:#?}"
    );
    assert!(errors[0].title.contains("crates/domain/types"));
}

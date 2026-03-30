use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_07_workspace_members_match_crate_dirs as assertions;

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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn wildcard_member_pattern_covers_non_hex_nested_package_paths() {
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
    "crates/app/rs/families/*/crates/runtime",
]
resolver = "2"
"#,
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/rs/families/deny/crates/runtime/Cargo.toml",
        "[package]\nname = \"devctl-rs-family-deny-runtime\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/rs/families/deny/crates/runtime/src/lib.rs",
        "// nested family runtime",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
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

    let results = super::run_family(tmp.path());
    assertions::assert_error_count(&results, "", 1);
    assertions::assert_any_result_contains_title(&results, &["crates/domain/types"]);
}

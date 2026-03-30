use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_27_nested_workspace_forbidden as assertions;

#[test]
fn nested_workspace_root_is_forbidden_even_inside_existing_leafs() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/types/examples/demo/Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\"]\nresolver = \"2\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/types/examples/demo/crates/runtime/Cargo.toml",
        "[package]\nname = \"demo-runtime\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/types/examples/demo/crates/runtime/src/lib.rs",
        "// demo runtime",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/crates/domain/types/examples/demo/Cargo.toml"),
            file_contains: None,
            title_contains: Some(&["nested workspace", "crates/domain/types/examples/demo"]),
            message_contains: None,
        }],
    );
}

#[test]
fn nested_package_root_does_not_hit_rule_27() {
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
    assertions::assert_no_error(&results);
}

#[test]
fn nested_workspace_root_still_hits_when_listed_in_workspace_members() {
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
    "crates/app/rs/families/deny",
]
resolver = "2"
"#,
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/rs/families/deny/Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\"]\nresolver = \"2\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/rs/families/deny/crates/runtime/Cargo.toml",
        "[package]\nname = \"devctl-rs-family-deny-runtime\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/rs/families/deny/crates/runtime/src/lib.rs",
        "// deny runtime",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_count(&results, 1);
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/crates/app/rs/families/deny/Cargo.toml"),
            file_contains: None,
            title_contains: Some(&["crates/app/rs/families/deny"]),
            message_contains: Some(&["only workspace root"]),
        }],
    );
}

#[test]
fn excluded_target_nested_workspace_does_not_hit_rule_27() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/target/family/Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\"]\nresolver = \"2\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/target/family/crates/runtime/Cargo.toml",
        "[package]\nname = \"target-runtime\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/target/family/crates/runtime/src/lib.rs",
        "// target runtime",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results);
}

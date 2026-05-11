use crate::avoid_breaking_exported_api::check;
use g3rs_clippy_config_checks_assertions::avoid_breaking_exported_api as assertions;
use g3rs_toml_parser::types::RustProfile;
use test_support::{
    cargo_member, cargo_root, input_with_raw, missing_cargo_root, parsed_rust_policy,
};

#[test]
fn warns_when_enabled_for_non_library_policy() {
    let input = input_with_raw(
        "clippy.toml",
        "avoid-breaking-exported-api = true\n",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Service), true),
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        assertions::findings(&results),
        vec![assertions::warn(
            "avoid-breaking-exported-api enabled",
            "`avoid-breaking-exported-api = true` suppresses useful lints. Prefer `false`.",
            "clippy.toml",
            false,
        )]
    );
}

#[test]
fn inventories_enabled_setting_for_published_library_policy() {
    let input = input_with_raw(
        "clippy.toml",
        "avoid-breaking-exported-api = true\n",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Library), true),
        cargo_root(
            "Cargo.toml",
            r#"[package]
name = "lib"
version = "0.1.0"
edition = "2024"
"#,
        ),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "library keeps avoid-breaking-exported-api enabled",
            "Published library profile may legitimately keep `avoid-breaking-exported-api = true`.",
            "clippy.toml",
            true,
        )],
    );
}

#[test]
fn inventories_enabled_setting_for_published_workspace_member_library() {
    let input = input_with_raw(
        "clippy.toml",
        "avoid-breaking-exported-api = true\n",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Library), true),
        cargo_root(
            "Cargo.toml",
            r#"[workspace]
members = ["member"]

[workspace.package]
publish = true
"#,
        ),
        vec![cargo_member(
            "member",
            "member/Cargo.toml",
            r#"[package]
name = "member"
version = "0.1.0"
edition = "2024"
publish = { workspace = true }
"#,
        )],
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "library keeps avoid-breaking-exported-api enabled",
            "Published library profile may legitimately keep `avoid-breaking-exported-api = true`.",
            "clippy.toml",
            true,
        )],
    );
}

#[test]
fn inventories_explicit_false_setting() {
    let input = input_with_raw(
        "clippy.toml",
        "avoid-breaking-exported-api = false\n",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Service), true),
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "avoid-breaking-exported-api explicitly false",
            "`avoid-breaking-exported-api = false` is set.",
            "clippy.toml",
            true,
        )],
    );
}

#[test]
fn warns_when_setting_is_missing() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Service), true),
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "avoid-breaking-exported-api not set",
            "Set `avoid-breaking-exported-api = false` explicitly unless this is a published library.",
            "clippy.toml",
            false,
        )],
    );
}

#[test]
fn warns_when_setting_has_wrong_type() {
    let input = input_with_raw(
        "clippy.toml",
        "avoid-breaking-exported-api = 7\n",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Service), true),
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "avoid-breaking-exported-api wrong type",
            "`avoid-breaking-exported-api` must be a bool, found integer.",
            "clippy.toml",
            false,
        )],
    );
}

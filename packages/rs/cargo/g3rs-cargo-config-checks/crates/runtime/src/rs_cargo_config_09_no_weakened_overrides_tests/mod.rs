use guardrail3_check_types::G3Severity;

use crate::test_support::{member, root};

#[test]
fn errors_when_member_weakens_workspace_lints() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"
        "#,
        None,
        false,
        Vec::new(),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true

            [lints.rust]
            warnings = "allow"
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_09_no_weakened_overrides::check(&root, &member, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-09");
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "weakened member rust override");
}

#[test]
fn inventories_when_member_does_not_weaken_workspace_lints() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            unwrap_used = "deny"
        "#,
        None,
        false,
        Vec::new(),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true

            [lints.rust]
            warnings = "deny"
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_09_no_weakened_overrides::check(&root, &member, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-09");
    assert_eq!(result.severity(), G3Severity::Info);
    assert_eq!(result.title(), "no weakened overrides");
    assert!(result.inventory());
}

#[test]
fn errors_when_member_lint_table_shape_is_invalid() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            unwrap_used = "deny"
        "#,
        None,
        false,
        Vec::new(),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true

            [lints.rust]
            warnings = []
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_09_no_weakened_overrides::check(&root, &member, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-09");
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "invalid member rust override");
}

#[test]
fn errors_when_member_weakens_forbid_to_deny() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.lints.rust]
            unsafe_code = "forbid"

            [workspace.lints.clippy]
            unwrap_used = "deny"
        "#,
        None,
        false,
        Vec::new(),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true

            [lints.rust]
            unsafe_code = "deny"
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_09_no_weakened_overrides::check(&root, &member, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "weakened member rust override");
}

#[test]
fn stays_quiet_when_workspace_policy_is_incomplete() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"
        "#,
        None,
        false,
        Vec::new(),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_09_no_weakened_overrides::check(&root, &member, &mut results);

    assert!(results.is_empty(), "{results:#?}");
}

use g3rs_cargo_config_checks_assertions::rs_cargo_config_09_no_weakened_overrides as assertions;
use test_support::{member, parsed_rust_policy, root};

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
        parsed_rust_policy(None, Vec::new()),
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

    assertions::assert_has_error(&results, "weakened member rust override", false);
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
        parsed_rust_policy(None, Vec::new()),
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

    assertions::assert_has_info(&results, "no weakened overrides", true);
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
        parsed_rust_policy(None, Vec::new()),
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

    assertions::assert_has_error(&results, "invalid member rust override", false);
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
        parsed_rust_policy(None, Vec::new()),
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

    assertions::assert_has_error(&results, "weakened member rust override", false);
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
        parsed_rust_policy(None, Vec::new()),
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

    assertions::assert_no_findings(&results);
}

use g3rs_cargo_config_checks_assertions::rs_cargo_config_12_member_local_allows_forbidden as assertions;
use test_support::{member, parsed_rust_policy, root, waiver};

#[test]
fn errors_on_member_local_allow_entries() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            all = { level = "deny", priority = -1 }
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

            [lints.clippy]
            module_name_repetitions = "allow"
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_12_member_local_allows_forbidden::check(&root, &member, &mut results);

    assertions::assert_has_error(&results, "member-local allow entry missing reason", false);
}

#[test]
fn errors_on_documented_member_local_allow_entries() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            all = { level = "deny", priority = -1 }
        "#,
        parsed_rust_policy(
            None,
            vec![waiver(
                "RS-CARGO-CONFIG-12",
                "crates/api/Cargo.toml",
                "clippy:module_name_repetitions",
                "Temporary lint suppression while API cleanup lands.",
            )],
        ),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true

            [lints.clippy]
            module_name_repetitions = "allow"
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_12_member_local_allows_forbidden::check(&root, &member, &mut results);

    assertions::assert_has_error(&results, "member-local allow entry forbidden", false);
}

#[test]
fn inventories_when_member_has_no_local_allow_entries() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            all = { level = "deny", priority = -1 }
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

    crate::rs_cargo_config_12_member_local_allows_forbidden::check(&root, &member, &mut results);

    assertions::assert_has_info(&results, "no member-local allow entries", true);
}

#[test]
fn errors_when_member_local_allow_reason_is_too_weak() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            all = { level = "deny", priority = -1 }
        "#,
        parsed_rust_policy(
            None,
            vec![waiver(
                "RS-CARGO-CONFIG-12",
                "crates/api/Cargo.toml",
                "clippy:module_name_repetitions",
                "temp",
            )],
        ),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true

            [lints.clippy]
            module_name_repetitions = "allow"
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_12_member_local_allows_forbidden::check(&root, &member, &mut results);

    assertions::assert_has_error(&results, "member-local allow entry reason too weak", false);
}

#[test]
fn stays_quiet_when_member_override_shape_is_invalid() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            all = { level = "deny", priority = -1 }
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
            clippy = "bad"
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_12_member_local_allows_forbidden::check(&root, &member, &mut results);

    assertions::assert_no_findings(&results);
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

    crate::rs_cargo_config_12_member_local_allows_forbidden::check(&root, &member, &mut results);

    assertions::assert_no_findings(&results);
}

#[test]
fn stands_down_when_rust_policy_parse_error_blocks_member_reason_resolution() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            all = { level = "deny", priority = -1 }
        "#,
        test_support::parse_error_rust_policy("bad rust policy"),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true

            [lints.clippy]
            module_name_repetitions = "allow"
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_12_member_local_allows_forbidden::check(&root, &member, &mut results);

    assertions::assert_no_findings(&results);
}

#[test]
fn stands_down_when_rust_policy_is_unreadable() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            all = { level = "deny", priority = -1 }
        "#,
        g3rs_cargo_types::G3RsCargoRustPolicyState::Unreadable {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "file is not readable".to_owned(),
        },
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = true

            [lints.clippy]
            module_name_repetitions = "allow"
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_12_member_local_allows_forbidden::check(&root, &member, &mut results);

    assertions::assert_no_findings(&results);
}

use g3rs_cargo_config_checks_assertions::rs_cargo_config_11_unapproved_allow_entries as assertions;
use test_support::{parse_error_rust_policy, parsed_rust_policy, root, waiver};

#[test]
fn errors_on_unapproved_allow_entries() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints.rust]
            warnings = "allow"
        "#,
        parsed_rust_policy(None, Vec::new()),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_11_unapproved_allow_entries::check(&root, &mut results);

    assertions::assert_has_error(&results, "unapproved allow entry missing reason", false);
}

#[test]
fn errors_on_hybrid_root_package_allow_entries() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            all = { level = "deny", priority = -1 }

            [package]
            name = "hybrid"
            version = "0.1.0"
            edition = "2024"

            [lints.rust]
            warnings = "allow"
        "#,
        parsed_rust_policy(None, Vec::new()),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_11_unapproved_allow_entries::check(&root, &mut results);

    assertions::assert_has_error(&results, "unapproved allow entry missing reason", false);
}

#[test]
fn errors_on_documented_unapproved_allow_entries() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints.rust]
            warnings = "allow"
        "#,
        parsed_rust_policy(
            None,
            vec![waiver(
                "RS-CARGO-CONFIG-11",
                "Cargo.toml",
                "rust:warnings",
                "Temporary lint suppression while API cleanup lands.",
            )],
        ),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_11_unapproved_allow_entries::check(&root, &mut results);

    assertions::assert_has_error(&results, "unapproved allow entry", false);
}

#[test]
fn inventories_when_no_unapproved_allow_entries_exist() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            module_name_repetitions = "allow"
        "#,
        parsed_rust_policy(None, Vec::new()),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_11_unapproved_allow_entries::check(&root, &mut results);

    assertions::assert_has_info(&results, "no unapproved allow entries", true);
}

#[test]
fn errors_when_unapproved_allow_reason_is_too_weak() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints.rust]
            warnings = "allow"
        "#,
        parsed_rust_policy(
            None,
            vec![waiver(
                "RS-CARGO-CONFIG-11",
                "Cargo.toml",
                "rust:warnings",
                "temp",
            )],
        ),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_11_unapproved_allow_entries::check(&root, &mut results);

    assertions::assert_has_error(&results, "unapproved allow entry reason too weak", false);
}

#[test]
fn stays_quiet_when_rust_policy_parse_error_suppresses_clean_inventory() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints.rust]
            warnings = "deny"
        "#,
        parse_error_rust_policy("bad rust policy"),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_11_unapproved_allow_entries::check(&root, &mut results);

    assertions::assert_no_findings(&results);
}

#[test]
fn stands_down_when_rust_policy_parse_error_blocks_reason_resolution() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints.rust]
            warnings = "allow"
        "#,
        parse_error_rust_policy("bad rust policy"),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_11_unapproved_allow_entries::check(&root, &mut results);

    assertions::assert_no_findings(&results);
}

#[test]
fn stands_down_when_rust_policy_is_unreadable() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints.rust]
            warnings = "allow"
        "#,
        g3rs_cargo_types::G3RsCargoRustPolicyState::Unreadable {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "file is not readable".to_owned(),
        },
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_11_unapproved_allow_entries::check(&root, &mut results);

    assertions::assert_no_findings(&results);
}

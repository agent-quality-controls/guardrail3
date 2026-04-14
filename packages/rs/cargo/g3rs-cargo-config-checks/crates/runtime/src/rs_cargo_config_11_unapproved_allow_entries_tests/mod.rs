use guardrail3_check_types::G3Severity;

use crate::test_support::{parse_error_rust_policy, parsed_rust_policy, root, waiver};

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

    let result = results
        .iter()
        .find(|result| result.title() == "unapproved allow entry missing reason")
        .unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-11");
    assert_eq!(result.severity(), G3Severity::Error);
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

    let result = results
        .iter()
        .find(|result| result.title() == "unapproved allow entry")
        .unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-11");
    assert_eq!(result.severity(), G3Severity::Error);
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

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-11");
    assert_eq!(result.severity(), G3Severity::Info);
    assert_eq!(result.title(), "no unapproved allow entries");
    assert!(result.inventory());
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
            vec![waiver("RS-CARGO-CONFIG-11", "Cargo.toml", "rust:warnings", "temp")],
        ),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_11_unapproved_allow_entries::check(&root, &mut results);

    let result = results
        .iter()
        .find(|result| result.title() == "unapproved allow entry reason too weak")
        .unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-11");
    assert_eq!(result.severity(), G3Severity::Error);
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

    assert!(results.is_empty(), "{results:#?}");
}

use g3rs_cargo_config_checks_assertions::rs_cargo_config_07_approved_allow_inventory as assertions;
use g3rs_cargo_types::G3RsCargoRustPolicyState;
use test_support::{parse_error_rust_policy, parsed_rust_policy, root, waiver};

#[test]
fn inventories_documented_approved_allow_entries() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints.clippy]
            module_name_repetitions = "allow"
            must_use_candidate = "allow"
        "#,
        parsed_rust_policy(
            None,
            vec![waiver(
                "RS-CARGO-CONFIG-07",
                "Cargo.toml",
                "clippy:module_name_repetitions",
                "Temporary lint suppression while API cleanup lands.",
            )],
        ),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_07_approved_allow_inventory::check(&root, &mut results);

    assertions::assert_has_warn(&results, "approved allow entry", false);
    assertions::assert_has_error(&results, "approved allow entry missing reason", false);
    assertions::assert_has_warn(&results, "approved allow count", false);
    assertions::assert_message_contains(
        &results,
        "approved allow count",
        "2 approved manifest allow entries (1 documented, 1 missing reasons, 0 weak reasons)",
    );
}

#[test]
fn errors_when_approved_allow_reason_is_too_weak() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints.clippy]
            module_name_repetitions = "allow"
        "#,
        parsed_rust_policy(
            None,
            vec![waiver(
                "RS-CARGO-CONFIG-07",
                "Cargo.toml",
                "clippy:module_name_repetitions",
                "temp",
            )],
        ),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_07_approved_allow_inventory::check(&root, &mut results);

    assertions::assert_has_error(&results, "approved allow entry reason too weak", false);
}

#[test]
fn stays_quiet_when_clippy_table_shape_is_invalid() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints]
            clippy = "bad"
        "#,
        G3RsCargoRustPolicyState::Missing,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_07_approved_allow_inventory::check(&root, &mut results);

    assertions::assert_no_findings(&results);
}

#[test]
fn stands_down_when_rust_policy_parse_error_blocks_waiver_resolution() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints.clippy]
            module_name_repetitions = "allow"
        "#,
        parse_error_rust_policy("bad rust policy"),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_07_approved_allow_inventory::check(&root, &mut results);

    assertions::assert_no_findings(&results);
}

#[test]
fn stands_down_when_rust_policy_is_unreadable() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints.clippy]
            module_name_repetitions = "allow"
        "#,
        G3RsCargoRustPolicyState::Unreadable {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "file is not readable".to_owned(),
        },
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_07_approved_allow_inventory::check(&root, &mut results);

    assertions::assert_no_findings(&results);
}

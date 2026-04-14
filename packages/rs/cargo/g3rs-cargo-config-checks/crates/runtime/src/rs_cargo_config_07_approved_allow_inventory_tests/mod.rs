use cargo_toml_parser::parse as parse_cargo_toml;
use g3rs_cargo_types::{
    G3RsCargoPolicyRoot, G3RsCargoPolicyRootKind, G3RsCargoRustPolicyState,
};
use guardrail3_check_types::G3Severity;

use crate::test_support::{parse_error_rust_policy, parsed_rust_policy, root, waiver};

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

    let documented = results
        .iter()
        .find(|result| result.title() == "approved allow entry")
        .unwrap();
    assert_eq!(documented.id(), "RS-CARGO-CONFIG-07");
    assert_eq!(documented.severity(), G3Severity::Warn);

    let missing_reason = results
        .iter()
        .find(|result| result.title() == "approved allow entry missing reason")
        .unwrap();
    assert_eq!(missing_reason.severity(), G3Severity::Error);

    let summary = results
        .iter()
        .find(|result| result.title() == "approved allow count")
        .unwrap();
    assert_eq!(summary.severity(), G3Severity::Warn);
    assert!(
        summary
            .message()
            .contains("2 approved manifest allow entries (1 documented, 1 missing reasons, 0 weak reasons)"),
        "{results:#?}"
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

    let weak = results
        .iter()
        .find(|result| result.title() == "approved allow entry reason too weak")
        .unwrap();
    assert_eq!(weak.id(), "RS-CARGO-CONFIG-07");
    assert_eq!(weak.severity(), G3Severity::Error);
}

#[test]
fn stays_quiet_when_clippy_table_shape_is_invalid() {
    let cargo = parse_cargo_toml(
        r#"
            [workspace]
            members = []
            resolver = "2"
        "#,
    )
    .expect("typed cargo fixture should parse");
    let raw_cargo = toml::from_str::<toml::Value>(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.lints]
            clippy = "bad"
        "#,
    )
    .expect("raw cargo fixture should parse");
    let root = G3RsCargoPolicyRoot {
        kind: G3RsCargoPolicyRootKind::WorkspaceRoot,
        rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo,
        raw_cargo,
        rust_policy: G3RsCargoRustPolicyState::Missing,
        edition: None,
        edition_invalid: false,
        rust_version: None,
        rust_version_invalid: false,
    };
    let mut results = Vec::new();

    crate::rs_cargo_config_07_approved_allow_inventory::check(&root, &mut results);

    assert!(results.is_empty(), "{results:#?}");
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

    assert!(results.is_empty(), "{results:#?}");
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

    assert!(results.is_empty(), "{results:#?}");
}

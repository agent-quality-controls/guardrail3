use cargo_toml_parser::parse as parse_cargo_toml;
use g3rs_cargo_types::{G3RsCargoPolicyRoot, G3RsCargoPolicyRootKind};
use guardrail3_check_types::G3Severity;

use crate::test_support::root;

#[test]
fn errors_when_library_profile_has_no_rust_version() {
    let root = root(
        r#"
            [package]
            name = "pkg"
            edition = "2024"
        "#,
        Some("library"),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_13_rust_version_policy::check(&root, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-13");
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "library rust-version missing");
}

#[test]
fn inventories_when_library_profile_declares_rust_version() {
    let root = root(
        r#"
            [package]
            name = "pkg"
            edition = "2024"
            rust-version = "1.84"
        "#,
        Some("library"),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_13_rust_version_policy::check(&root, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-13");
    assert_eq!(result.severity(), G3Severity::Info);
    assert_eq!(result.title(), "library rust-version declared");
    assert!(result.inventory());
}

#[test]
fn errors_when_rust_version_shape_is_invalid() {
    let cargo = parse_cargo_toml(
        r#"
            [package]
            name = "pkg"
            edition = "2024"
        "#,
    )
    .expect("typed cargo fixture should parse");
    let raw_cargo = toml::from_str::<toml::Value>(
        r#"
            [package]
            name = "pkg"
            edition = "2024"
            rust-version = []
        "#,
    )
    .expect("raw cargo fixture should parse");
    let root = G3RsCargoPolicyRoot {
        kind: G3RsCargoPolicyRootKind::StandalonePackageRoot,
        rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo,
        raw_cargo,
        guardrail_rel_path: Some("guardrail3.toml".to_owned()),
        profile_name: Some("library".to_owned()),
        escape_hatches: Vec::new(),
        guardrail_parse_error: false,
        edition: Some("2024".to_owned()),
        edition_invalid: false,
        rust_version: None,
        rust_version_invalid: true,
    };
    let mut results = Vec::new();

    crate::rs_cargo_config_13_rust_version_policy::check(&root, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-13");
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "rust-version invalid");
}

#[test]
fn inventories_when_non_library_omits_rust_version() {
    let root = root(
        r#"
            [package]
            name = "pkg"
            edition = "2024"
        "#,
        Some("service"),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_13_rust_version_policy::check(&root, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-13");
    assert_eq!(result.severity(), G3Severity::Info);
    assert_eq!(result.title(), "rust-version inventory");
    assert!(result.inventory());
}

#[test]
fn inventories_when_workspace_root_library_declares_rust_version() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            rust-version = "1.84"
        "#,
        Some("library"),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_13_rust_version_policy::check(&root, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-13");
    assert_eq!(result.severity(), G3Severity::Info);
    assert_eq!(result.title(), "library rust-version declared");
    assert!(result.inventory());
}

#[test]
fn errors_when_workspace_root_rust_version_shape_is_invalid() {
    let cargo = parse_cargo_toml(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [package]
            name = "pkg"
            edition = "2024"
            rust-version = "1.84"
        "#,
    )
    .expect("typed cargo fixture should parse");
    let raw_cargo = toml::from_str::<toml::Value>(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            rust-version = []

            [package]
            name = "pkg"
            edition = "2024"
            rust-version = "1.84"
        "#,
    )
    .expect("raw cargo fixture should parse");
    let root = G3RsCargoPolicyRoot {
        kind: G3RsCargoPolicyRootKind::WorkspaceRoot,
        rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo,
        raw_cargo,
        guardrail_rel_path: Some("guardrail3.toml".to_owned()),
        profile_name: Some("library".to_owned()),
        escape_hatches: Vec::new(),
        guardrail_parse_error: false,
        edition: Some("2024".to_owned()),
        edition_invalid: false,
        rust_version: Some("1.84".to_owned()),
        rust_version_invalid: true,
    };
    let mut results = Vec::new();

    crate::rs_cargo_config_13_rust_version_policy::check(&root, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-13");
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "rust-version invalid");
}

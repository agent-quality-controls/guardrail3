use cargo_toml_parser::parse as parse_cargo_toml;
use g3rs_cargo_config_checks_assertions::rs_cargo_config_10_member_edition_drift as assertions;
use g3rs_cargo_types::{G3RsCargoPolicyRoot, G3RsCargoPolicyRootKind};
use test_support::{member, parsed_rust_policy, root};

#[test]
fn warns_when_member_edition_is_older_than_workspace() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = "2024"
        "#,
        parsed_rust_policy(None, Vec::new()),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2021"

            [lints]
            workspace = true
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_10_member_edition_drift::check(&root, &member, &mut results);

    assertions::assert_has_warn(&results, "member edition older than workspace", false);
}

#[test]
fn inventories_when_member_inherits_workspace_edition() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = "2024"
        "#,
        parsed_rust_policy(None, Vec::new()),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"

            [lints]
            workspace = true
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_10_member_edition_drift::check(&root, &member, &mut results);

    assertions::assert_has_info(&results, "member inherits workspace edition", true);
}

#[test]
fn errors_when_member_edition_is_invalid() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = "2024"
        "#,
        parsed_rust_policy(None, Vec::new()),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = []

            [lints]
            workspace = true
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_10_member_edition_drift::check(&root, &member, &mut results);

    assertions::assert_has_error(&results, "member edition invalid", false);
}

#[test]
fn errors_when_member_edition_is_unrecognized() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = "2024"
        "#,
        parsed_rust_policy(None, Vec::new()),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "3021"

            [lints]
            workspace = true
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_10_member_edition_drift::check(&root, &member, &mut results);

    assertions::assert_has_error(&results, "member edition unrecognized", false);
}

#[test]
fn stays_quiet_when_workspace_has_no_edition_policy() {
    let root = root(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"
        "#,
        parsed_rust_policy(None, Vec::new()),
    );
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2021"

            [lints]
            workspace = true
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_10_member_edition_drift::check(&root, &member, &mut results);

    assertions::assert_no_findings(&results);
}

#[test]
fn stays_quiet_when_workspace_edition_shape_is_invalid_even_if_package_has_valid_fallback() {
    let cargo = parse_cargo_toml(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [package]
            name = "hybrid"
            version = "0.1.0"
            edition = "2024"
        "#,
    )
    .expect("typed cargo fixture should parse");
    let raw_cargo = toml::from_str::<toml::Value>(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = []

            [package]
            name = "hybrid"
            version = "0.1.0"
            edition = "2024"
        "#,
    )
    .expect("raw cargo fixture should parse");
    let root = G3RsCargoPolicyRoot {
        kind: G3RsCargoPolicyRootKind::WorkspaceRoot,
        rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo,
        raw_cargo,
        rust_policy: g3rs_cargo_types::G3RsCargoRustPolicyState::Missing,
        edition: Some("2024".to_owned()),
        edition_invalid: true,
        rust_version: None,
        rust_version_invalid: false,
    };
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2021"
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_10_member_edition_drift::check(&root, &member, &mut results);

    assertions::assert_no_findings(&results);
}

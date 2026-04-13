use guardrail3_check_types::G3Severity;

use crate::test_support::member;

#[test]
fn errors_when_member_does_not_inherit_workspace_lints() {
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2024"
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_08_workspace_lints_inherited::check(&member, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-08");
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "workspace lints not inherited");
}

#[test]
fn inventories_when_member_inherits_workspace_lints() {
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

    crate::rs_cargo_config_08_workspace_lints_inherited::check(&member, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-08");
    assert_eq!(result.severity(), G3Severity::Info);
    assert_eq!(result.title(), "workspace lints inherited");
    assert!(result.inventory());
}

#[test]
fn errors_when_workspace_lint_inheritance_shape_is_invalid() {
    let member = member(
        "crates/api",
        r#"
            [package]
            name = "api"
            edition = "2024"

            [lints]
            workspace = "yes"
        "#,
    );
    let mut results = Vec::new();

    crate::rs_cargo_config_08_workspace_lints_inherited::check(&member, &mut results);

    let result = results.first().unwrap();
    assert_eq!(result.id(), "RS-CARGO-CONFIG-08");
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "workspace lint inheritance invalid");
}

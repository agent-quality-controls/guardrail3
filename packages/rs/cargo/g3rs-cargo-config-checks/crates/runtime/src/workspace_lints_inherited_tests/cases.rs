use g3rs_cargo_config_checks_assertions::workspace_lints_inherited as assertions;
use test_support::member;

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

    crate::workspace_lints_inherited::check(&member, &mut results);

    assertions::assert_has_error(&results, "workspace lints not inherited", false);
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

    crate::workspace_lints_inherited::check(&member, &mut results);

    assertions::assert_has_info(&results, "workspace lints inherited", true);
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

    crate::workspace_lints_inherited::check(&member, &mut results);

    assertions::assert_has_error(&results, "workspace lint inheritance invalid", false);
}

use super::helpers::run_check_with_local_paths;

#[test]
fn local_cargo_path_dependency_uses_target_package_name_for_allowlist() {
    let results = run_check_with_local_paths(
        r#"
            [workspace]
            members = ["apps/api"]
        "#,
        "apps/api/Cargo.toml",
        r#"
            [package]
            name = "api"

            [dependencies]
            vendored = { path = "../../../vendor/serde_pkg" }
        "#,
        r#"
            [rust.apps.api]
            profile = "service"
            allowed_deps = ["serde"]
        "#,
        &["../vendor/serde_pkg/Cargo.toml"],
        &[(
            "../vendor/serde_pkg/Cargo.toml",
            r#"
                [package]
                name = "serde"
            "#,
        )],
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-DEPS-05");
    assert!(results[0].inventory());
    assert!(
        results[0].message().contains("Dependency `serde`"),
        "expected target package name in allowlist result: {results:#?}"
    );
}

#[test]
fn undeclared_local_cargo_package_under_workspace_root_is_skipped_by_content_rule() {
    let results = run_check_with_local_paths(
        r#"
            [workspace]
            members = ["apps/api"]
        "#,
        "apps/api/Cargo.toml",
        r#"
            [package]
            name = "api"

            [dependencies]
            vendored = { path = "../../vendor/serde_pkg" }
        "#,
        r#"
            [rust.apps.api]
            profile = "service"
            allowed_deps = ["serde"]
        "#,
        &["vendor/serde_pkg/Cargo.toml"],
        &[(
            "vendor/serde_pkg/Cargo.toml",
            r#"
                [package]
                name = "serde"
            "#,
        )],
    );

    assert!(
        results.is_empty(),
        "content rule should stand down and let app-owned RS-DEPS-11 report this case: {results:#?}"
    );
}

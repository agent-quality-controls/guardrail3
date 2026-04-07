use super::helpers::run_check_with_local_paths;

#[test]
fn local_cargo_path_dependency_deduplicates_by_target_package_name() {
    let mut deps = String::new();
    for idx in 0..24 {
        deps.push_str(&format!("dep_{idx} = \"1\"\n"));
    }
    deps.push_str("serde = \"1\"\n");
    deps.push_str("vendored = { path = \"../../../vendor/serde_pkg\" }\n");

    let crate_manifest = format!(
        "[package]\nname = \"api\"\n\n[dependencies]\n{deps}"
    );

    let results = run_check_with_local_paths(
        r#"
            [workspace]
            members = ["apps/api"]
        "#,
        &crate_manifest,
        &["../vendor/serde_pkg/Cargo.toml"],
        &[(
            "../vendor/serde_pkg/Cargo.toml",
            r#"
                [package]
                name = "serde"
            "#,
        )],
    );

    assert!(
        results.is_empty(),
        "deduplicated local path package name should keep the crate at the cap, not over it: {results:#?}"
    );
}

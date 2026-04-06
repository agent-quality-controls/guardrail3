use super::helpers::run_check;

#[test]
fn inventories_allowlisted_runtime_dependency() {
    let results = run_check(
        "[workspace]\nmembers = [\"packages/core\"]\n",
        "packages/core/Cargo.toml",
        "[package]\nname = \"core\"\n\n[dependencies]\nserde = \"1\"\n",
        "[profile]\nname = \"library\"\n[rust.packages]\nallowed_deps = [\"serde\"]\n",
    );

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-01"
            && result.inventory()
            && result.message().contains("Dependency `serde`")
    }));
}

#[test]
fn renamed_dependency_uses_package_name_for_allowlist() {
    let results = run_check(
        "[workspace]\nmembers = [\"packages/core\"]\n",
        "packages/core/Cargo.toml",
        "[package]\nname = \"core\"\n\n[dependencies]\nserde_alias = { package = \"serde\", version = \"1\" }\n",
        "[profile]\nname = \"library\"\n[rust.packages]\nallowed_deps = [\"serde\"]\n",
    );

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-01"
            && result.inventory()
            && result.message().contains("Dependency `serde`")
    }));
}

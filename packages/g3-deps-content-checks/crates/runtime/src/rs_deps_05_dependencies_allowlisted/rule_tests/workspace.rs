use super::helpers::run_check;

#[test]
fn workspace_true_external_path_dependency_is_checked() {
    let results = run_check(
        "[workspace]\nmembers = [\"packages/*\"]\n\n[workspace.dependencies]\nvendored_reqwest = { package = \"reqwest\", path = \"vendor/reqwest\" }\n",
        "packages/core/Cargo.toml",
        "[package]\nname = \"core\"\n\n[dependencies]\nvendored_reqwest = { workspace = true }\n",
        "[profile]\nname = \"library\"\n[rust.packages]\nallowed_deps = [\"serde\"]\n",
    );

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-05"
            && matches!(result.severity(), guardrail3_check_types::G3Severity::Error)
            && result.message().contains("Dependency `reqwest`")
    }));
}

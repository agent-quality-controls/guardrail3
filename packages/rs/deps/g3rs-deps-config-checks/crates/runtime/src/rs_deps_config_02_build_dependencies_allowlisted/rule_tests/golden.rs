use super::helpers::run_check;

#[test]
fn workspace_true_external_build_dependency_is_checked() {
    let results = run_check(
        "[workspace]\nmembers = [\"packages/*\"]\n\n[workspace.dependencies]\nbindgen = \"0.70\"\n",
        "[package]\nname = \"core\"\n\n[build-dependencies]\nbindgen = { workspace = true }\n",
        "[profile]\nname = \"library\"\n[rust.packages]\nallowed_deps = [\"serde\"]\n",
    );

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-02"
            && matches!(result.severity(), guardrail3_check_types::G3Severity::Error)
            && result.message().contains("Build dependency `bindgen`")
    }));
}

#[test]
fn build_rule_stays_silent_without_allowlist() {
    let results = run_check(
        "[workspace]\nmembers = [\"packages/*\"]\n",
        "[package]\nname = \"core\"\n\n[build-dependencies]\nbindgen = \"0.70\"\n",
        "[profile]\nname = \"library\"\n",
    );

    assert!(results.is_empty());
}

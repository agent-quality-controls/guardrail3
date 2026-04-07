use super::helpers::run_check;

#[test]
fn workspace_true_external_dev_dependency_keeps_warn_severity() {
    let results = run_check(
        "[workspace]\nmembers = [\"packages/*\"]\n\n[workspace.dependencies]\ntempfile = \"3\"\n",
        "[package]\nname = \"core\"\n\n[dev-dependencies]\ntempfile = { workspace = true }\n",
        "[profile]\nname = \"library\"\n[rust.packages]\nallowed_deps = [\"serde\"]\n",
    );

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-03"
            && matches!(result.severity(), guardrail3_check_types::G3Severity::Warn)
            && result.message().contains("Dev dependency `tempfile`")
    }));
}

#[test]
fn dev_rule_stays_silent_without_allowlist() {
    let results = run_check(
        "[workspace]\nmembers = [\"packages/*\"]\n",
        "[package]\nname = \"core\"\n\n[dev-dependencies]\ntempfile = \"3\"\n",
        "[profile]\nname = \"library\"\n",
    );

    assert!(results.is_empty());
}

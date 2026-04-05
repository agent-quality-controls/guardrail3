use super::helpers::run_check;

#[test]
fn deduplicates_package_names_across_sections_aliases_and_targets() {
    let unique_dependencies = (0..24)
        .map(|index| format!("dep{index} = \"1\""))
        .collect::<Vec<_>>()
        .join("\n");
    let manifest = format!(
        "[package]\nname = \"api\"\n\n[dependencies]\n{unique_dependencies}\nserde = \"1\"\n\n[build-dependencies]\nserde_build = {{ package = \"serde\", version = \"1\" }}\n\n[dev-dependencies]\nserde_dev = {{ package = \"serde\", version = \"1\" }}\n\n[target.'cfg(unix)'.dependencies]\nserde_unix = {{ package = \"serde\", version = \"1\" }}\n"
    );

    let results = run_check("[workspace]\nmembers = [\"apps/*\"]\n", &manifest);
    assert!(results.iter().all(|result| result.id() != "RS-DEPS-12"));
}

#[test]
fn counts_external_workspace_and_vendored_path_but_skips_internal_workspace_path() {
    let filler_dependencies = (0..23)
        .map(|index| format!("dep{index} = \"1\""))
        .collect::<Vec<_>>()
        .join("\n");
    let manifest = format!(
        "[package]\nname = \"api\"\n\n[dependencies]\n{filler_dependencies}\nsupport = {{ path = \"../support\" }}\nvendored_reqwest = {{ package = \"reqwest\", path = \"../../vendor/reqwest\" }}\nserde = {{ workspace = true }}\n\n[target.'cfg(unix)'.dependencies]\nbytes = \"1\"\n"
    );

    let results = run_check(
        "[workspace]\nmembers = [\"apps/*\"]\n\n[workspace.dependencies]\nserde = \"1\"\n",
        &manifest,
    );

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-12"
            && matches!(result.severity(), guardrail3_check_types::G3Severity::Error)
            && result.message().contains("Crate `api` has 26 unique direct dependencies")
    }));
}

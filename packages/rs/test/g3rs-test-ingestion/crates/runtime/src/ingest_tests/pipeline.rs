use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use guardrail3_check_types::G3CheckResult;
use tempfile::tempdir;

fn git_init(path: &Path) {
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
    assert!(status.success(), "git init should exit successfully");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture file");
}

fn run_config_pipeline(root: &Path, cargo_mutants_installed: bool) -> Vec<G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::run::ingest_for_config_checks_with_tool_state(&crawl, cargo_mutants_installed)
        .expect("config ingestion should succeed");
    inputs
        .iter()
        .flat_map(g3rs_test_config_checks::check)
        .collect()
}

fn findings_by_file(results: &[G3CheckResult]) -> BTreeMap<String, Vec<&G3CheckResult>> {
    let mut by_file = BTreeMap::<String, Vec<&G3CheckResult>>::new();
    for result in results {
        let key = result.file().unwrap_or("<none>").to_owned();
        by_file.entry(key).or_default().push(result);
    }
    by_file
}

#[test]
fn pipeline_reports_missing_mutation_and_nextest_surfaces() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[package]\n\
name = \"demo\"\n\
version = \"0.1.0\"\n\
edition = \"2024\"\n\
\n\
[profile.mutants]\n\
inherits = \"dev\"\n\
\n\
[dev-dependencies]\n\
tokio = { version = \"1\", features = [\"macros\"] }\n",
    );
    write(root.join("src/lib.rs"), "#[tokio::test]\nasync fn runs() {}\n");

    let results = run_config_pipeline(root, false);
    let by_file = findings_by_file(&results);

    assert!(
        by_file[".config/nextest.toml"]
            .iter()
            .any(|result| result.id() == "RS-TEST-09"),
        "{results:#?}"
    );
    assert!(
        by_file["Cargo.toml"]
            .iter()
            .any(|result| result.id() == "RS-TEST-11"),
        "{results:#?}"
    );
    assert!(
        by_file[".cargo/mutants.toml"]
            .iter()
            .any(|result| result.id() == "RS-TEST-12"),
        "{results:#?}"
    );
    assert!(
        by_file["Cargo.toml"]
            .iter()
            .any(|result| result.id() == "RS-TEST-13"),
        "{results:#?}"
    );
    assert!(
        by_file["Cargo.toml"]
            .iter()
            .any(|result| result.id() == "RS-TEST-14"),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_sane_config_as_inventory() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[package]\n\
name = \"demo\"\n\
version = \"0.1.0\"\n\
edition = \"2024\"\n\
\n\
[profile.mutants]\n\
inherits = \"dev\"\n\
\n\
[dev-dependencies]\n\
tokio = { version = \"1\", features = [\"macros\"] }\n",
    );
    write(root.join("src/lib.rs"), "#[tokio::test]\nasync fn runs() {}\n");
    write(root.join(".cargo/mutants.toml"), "timeout_multiplier = 2.0\n");
    write(
        root.join(".config/nextest.toml"),
        "[profile.default]\nslow-timeout = \"60s\"\nleak-timeout = \"100ms\"\n",
    );
    write(root.join(".githooks/pre-commit"), "cargo mutants --profile dev\n");

    let results = run_config_pipeline(root, true);
    let by_file = findings_by_file(&results);

    assert!(
        by_file[".config/nextest.toml"]
            .iter()
            .any(|result| result.id() == "RS-TEST-09"),
        "{results:#?}"
    );
    assert!(
        by_file["Cargo.toml"]
            .iter()
            .any(|result| result.id() == "RS-TEST-11"),
        "{results:#?}"
    );
    assert!(
        by_file[".cargo/mutants.toml"]
            .iter()
            .any(|result| result.id() == "RS-TEST-12"),
        "{results:#?}"
    );
    assert!(
        by_file["Cargo.toml"]
            .iter()
            .any(|result| result.id() == "RS-TEST-13"),
        "{results:#?}"
    );
    assert!(
        by_file[".githooks/pre-commit"]
            .iter()
            .any(|result| result.id() == "RS-TEST-14"),
        "{results:#?}"
    );
    assert!(
        by_file[".cargo/mutants.toml"]
            .iter()
            .any(|result| result.id() == "RS-TEST-15"),
        "{results:#?}"
    );
}

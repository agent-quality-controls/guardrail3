#![expect(
    clippy::disallowed_methods,
    reason = "test fixtures must call std::fs and std::process::Command directly to seed and tear down filesystem state"
)]
use std::path::Path;
use std::process::Command;

use g3rs_test_ingestion_assertions::ingest::run::assert_file_has_result;
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
        super::super::create_fixture_dir(parent).expect("create parent directory");
    }
    super::super::write_fixture(path.as_ref(), content).expect("write fixture file");
}

fn run_config_pipeline(
    root: &Path,
    cargo_mutants_installed: bool,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs =
        super::super::ingest_for_config_checks_with_tool_state(&crawl, cargo_mutants_installed)
            .expect("config ingestion should succeed");
    inputs
        .iter()
        .flat_map(g3rs_test_config_checks::check)
        .collect()
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
    write(
        root.join("src/lib.rs"),
        "#[tokio::test]\nasync fn runs() {}\n",
    );

    let results = run_config_pipeline(root, false);
    assert_file_has_result(
        &results,
        ".config/nextest.toml",
        "g3rs-test/nextest-timeouts",
    );
    assert_file_has_result(&results, "Cargo.toml", "g3rs-test/cargo-mutants-installed");
    assert_file_has_result(
        &results,
        ".cargo/mutants.toml",
        "g3rs-test/mutants-toml-exists",
    );
    assert_file_has_result(&results, "Cargo.toml", "g3rs-test/mutants-profile-present");
    assert_file_has_result(&results, "Cargo.toml", "g3rs-test/mutation-hook-present");
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
    write(
        root.join("src/lib.rs"),
        "#[tokio::test]\nasync fn runs() {}\n",
    );
    write(
        root.join(".cargo/mutants.toml"),
        "timeout_multiplier = 2.0\n",
    );
    write(
        root.join(".config/nextest.toml"),
        "[profile.default]\nslow-timeout = \"60s\"\nleak-timeout = \"100ms\"\n",
    );
    write(
        root.join(".githooks/pre-commit"),
        "cargo mutants --profile dev\n",
    );

    let results = run_config_pipeline(root, true);
    assert_file_has_result(
        &results,
        ".config/nextest.toml",
        "g3rs-test/nextest-timeouts",
    );
    assert_file_has_result(&results, "Cargo.toml", "g3rs-test/cargo-mutants-installed");
    assert_file_has_result(
        &results,
        ".cargo/mutants.toml",
        "g3rs-test/mutants-toml-exists",
    );
    assert_file_has_result(&results, "Cargo.toml", "g3rs-test/mutants-profile-present");
    assert_file_has_result(
        &results,
        ".githooks/pre-commit",
        "g3rs-test/mutation-hook-present",
    );
    assert_file_has_result(
        &results,
        ".cargo/mutants.toml",
        "g3rs-test/mutants-config-sane",
    );
}

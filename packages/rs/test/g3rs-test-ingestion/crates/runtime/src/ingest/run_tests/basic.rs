use std::path::Path;
use std::process::Command;

use g3rs_test_ingestion_assertions::ingest::run::find_root;
use g3rs_workspace_crawl::crawl;
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

#[test]
fn ingests_test_config_root_with_typed_files_and_activation_facts() {
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

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_config_checks_with_tool_state(&workspace_crawl, true)
        .expect("ingestion should succeed");
    assert_eq!(inputs.len(), 1, "{inputs:#?}");

    let input = find_root(&inputs, "");
    assert!(
        input.is_some(),
        "missing test root input ``; available inputs: {inputs:#?}"
    );
    let Some(input) = input else {
        return;
    };
    assert_eq!(input.cargo_rel_path, "Cargo.toml");
    assert_eq!(input.mutants_rel_path, ".cargo/mutants.toml");
    assert_eq!(input.nextest_rel_path, ".config/nextest.toml");
    assert!(input.has_tests, "{input:#?}");
    assert!(input.has_tokio_tests, "{input:#?}");
    assert!(input.tokio_dependency_present, "{input:#?}");
    assert!(input.cargo_mutants_installed, "{input:#?}");
    assert!(input.mutation_hook_active, "{input:#?}");
    assert_eq!(
        input.mutation_hook_files,
        vec![".githooks/pre-commit".to_owned()]
    );
    assert!(input.mutants_exists, "{input:#?}");
    assert!(input.mutants.is_some(), "{input:#?}");
    assert!(input.nextest.is_some(), "{input:#?}");
}

#[test]
fn only_ingests_owned_workspace_members() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[workspace]\n\
members = [\"crates/app\"]\n\
resolver = \"2\"\n",
    );
    write(
        root.join("crates/app/Cargo.toml"),
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("crates/app/src/lib.rs"),
        "#[test]\nfn smoke() {}\n",
    );
    write(
        root.join("vendor/nested/Cargo.toml"),
        "[package]\nname = \"nested\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("vendor/nested/src/lib.rs"),
        "#[test]\nfn hidden() {}\n",
    );

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_config_checks(&workspace_crawl)
        .expect("ingestion should succeed");

    assert_eq!(inputs.len(), 1, "{inputs:#?}");
    assert_eq!(inputs[0].root_rel_dir, "crates/app");
}

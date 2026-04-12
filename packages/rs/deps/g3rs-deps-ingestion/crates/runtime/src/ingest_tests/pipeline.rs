use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn git_init(path: &Path) {
    let _ = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture file");
}

fn write_executable(path: impl AsRef<Path>, content: &str) {
    write(path.as_ref(), content);
    let mut permissions = fs::metadata(path.as_ref())
        .expect("metadata should be readable")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path.as_ref(), permissions).expect("chmod should succeed");
}

#[test]
fn pipeline_reports_missing_dependency_allowlist_for_library() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/core\"]\nresolver = \"2\"\n",
    );
    write(root.join("guardrail3-rs.toml"), "profile = \"library\"\n");
    write(
        root.join("crates/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_deps_config_checks::check)
        .collect::<Vec<_>>();

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-DEPS-CONFIG-04"
                && result.title() == "dependency allowlist missing"
                && result.file() == Some("crates/core/Cargo.toml")
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_workspace_tool_presence_in_deps_config_lane() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/core\"]\nresolver = \"2\"\n",
    );
    write(root.join("guardrail3-rs.toml"), "profile = \"service\"\n");
    write(
        root.join("crates/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    let tools_dir = root.join("bin");
    write_executable(tools_dir.join("cargo-deny"), "#!/bin/sh\nexit 0\n");
    write_executable(tools_dir.join("cargo-machete"), "#!/bin/sh\nexit 0\n");
    write_executable(tools_dir.join("gitleaks"), "#!/bin/sh\nexit 0\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::run::ingest_for_config_checks_with_path(&crawl, Some(tools_dir.as_os_str()))
        .expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_deps_config_checks::check)
        .map(|result| {
            (
                result.id().to_owned(),
                result.severity(),
                result.title().to_owned(),
                result.file().map(str::to_owned),
                result.inventory(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        results,
        vec![
            (
                "RS-DEPS-CONFIG-06".to_owned(),
                guardrail3_check_types::G3Severity::Info,
                "cargo-deny installed".to_owned(),
                Some("Cargo.toml".to_owned()),
                true,
            ),
            (
                "RS-DEPS-CONFIG-07".to_owned(),
                guardrail3_check_types::G3Severity::Info,
                "cargo-machete installed".to_owned(),
                Some("Cargo.toml".to_owned()),
                true,
            ),
            (
                "RS-DEPS-CONFIG-08".to_owned(),
                guardrail3_check_types::G3Severity::Warn,
                "cargo-dupes missing".to_owned(),
                Some("Cargo.toml".to_owned()),
                false,
            ),
            (
                "RS-DEPS-CONFIG-09".to_owned(),
                guardrail3_check_types::G3Severity::Info,
                "gitleaks installed".to_owned(),
                Some("Cargo.toml".to_owned()),
                true,
            ),
        ],
        "{results:#?}"
    );
}

#[test]
fn pipeline_emits_one_workspace_tool_result_set_even_with_multiple_crates() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/core\", \"crates/support\"]\nresolver = \"2\"\n",
    );
    write(root.join("guardrail3-rs.toml"), "profile = \"service\"\n");
    write(
        root.join("crates/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.join("crates/support/Cargo.toml"),
        "[package]\nname = \"support\"\nversion = \"0.1.0\"\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_config_checks_with_path(&crawl, None).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_deps_config_checks::check)
        .map(|result| {
            (
                result.id().to_owned(),
                result.severity(),
                result.title().to_owned(),
                result.file().map(str::to_owned),
                result.inventory(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        results,
        vec![
            (
                "RS-DEPS-CONFIG-06".to_owned(),
                guardrail3_check_types::G3Severity::Error,
                "cargo-deny missing".to_owned(),
                Some("Cargo.toml".to_owned()),
                false,
            ),
            (
                "RS-DEPS-CONFIG-07".to_owned(),
                guardrail3_check_types::G3Severity::Error,
                "cargo-machete missing".to_owned(),
                Some("Cargo.toml".to_owned()),
                false,
            ),
            (
                "RS-DEPS-CONFIG-08".to_owned(),
                guardrail3_check_types::G3Severity::Warn,
                "cargo-dupes missing".to_owned(),
                Some("Cargo.toml".to_owned()),
                false,
            ),
            (
                "RS-DEPS-CONFIG-09".to_owned(),
                guardrail3_check_types::G3Severity::Error,
                "gitleaks missing".to_owned(),
                Some("Cargo.toml".to_owned()),
                false,
            ),
        ],
        "{results:#?}"
    );
}

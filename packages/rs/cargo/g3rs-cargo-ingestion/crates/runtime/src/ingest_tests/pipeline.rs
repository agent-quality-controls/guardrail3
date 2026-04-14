use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed in test fixture setup");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)
            .expect("should create parent directories for test fixture files");
    }
    fs::write(path, content).expect("should write test fixture file to disk");
}

fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    g3rs_workspace_crawl::crawl(root).expect("crawl should succeed on valid test workspace")
}

#[test]
fn config_pipeline_reports_old_app_allow_inventory_and_member_rules() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = "2024"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            all = { level = "deny", priority = -1 }
            module_name_repetitions = "allow"
            unwrap_used = "deny"
            disallowed_macros = "deny"
        "#,
    );
    write(
        root.join("crates/api/Cargo.toml"),
        r#"
            [package]
            name = "api"
            edition = "2021"

            [lints]
            workspace = true

            [lints.rust]
            warnings = "allow"
        "#,
    );
    write(
        root.join("guardrail3-rs.toml"),
        r#"
            profile = "library"

            [[waivers]]
            rule = "RS-CARGO-CONFIG-07"
            file = "Cargo.toml"
            selector = "clippy:module_name_repetitions"
            reason = "Temporary lint suppression while API cleanup lands."
        "#,
    );

    let input = crate::ingest_for_config_checks(&crawl(root)).expect("ingestion should succeed");
    let results = g3rs_cargo_config_checks::check(&input);

    assert!(
        results
            .iter()
            .any(|result| result.id() == "RS-CARGO-CONFIG-07"),
        "{results:#?}"
    );
    assert!(
        results
            .iter()
            .any(|result| result.id() == "RS-CARGO-CONFIG-09"),
        "{results:#?}"
    );
    assert!(
        results
            .iter()
            .any(|result| result.id() == "RS-CARGO-CONFIG-10"),
        "{results:#?}"
    );
    assert!(
        results
            .iter()
            .any(|result| result.id() == "RS-CARGO-CONFIG-13"),
        "{results:#?}"
    );
}

#[test]
fn config_pipeline_stands_down_allow_rules_when_guardrail3_rs_is_invalid() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = "2024"

            [workspace.lints.rust]
            warnings = "deny"

            [workspace.lints.clippy]
            all = { level = "deny", priority = -1 }
            module_name_repetitions = "allow"
        "#,
    );
    write(
        root.join("crates/api/Cargo.toml"),
        r#"
            [package]
            name = "api"
            edition = "2021"

            [lints]
            workspace = true

            [lints.rust]
            warnings = "allow"
        "#,
    );
    write(root.join("guardrail3-rs.toml"), "profile = [");

    let input = crate::ingest_for_config_checks(&crawl(root)).expect("ingestion should succeed");
    let results = g3rs_cargo_config_checks::check(&input);

    assert!(
        !results
            .iter()
            .any(|result| matches!(result.id(), "RS-CARGO-CONFIG-07" | "RS-CARGO-CONFIG-11" | "RS-CARGO-CONFIG-12")),
        "{results:#?}"
    );
}

#[test]
fn filetree_pipeline_reports_guardrail3_rs_parse_failures_and_ignores_legacy_guardrail3_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"
        "#,
    );
    write(root.join("crates/api/Cargo.toml"), "[package");
    write(root.join("guardrail3-rs.toml"), "profile = [");
    write(root.join("guardrail3.toml"), "[profile]\nname = \"library\"\n");

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed with fail-closed inputs");
    let results = g3rs_cargo_filetree_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CARGO-FILETREE-14"
                && result.file() == Some("guardrail3-rs.toml")
        }),
        "{results:#?}"
    );
    assert!(
        !results.iter().any(|result| {
            result.id() == "RS-CARGO-FILETREE-14"
                && result.file() == Some("guardrail3.toml")
        }),
        "{results:#?}"
    );
}

#[test]
fn filetree_pipeline_reports_missing_member_and_input_failures() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api", "crates/missing"]
            resolver = "2"
        "#,
    );
    write(root.join("crates/api/Cargo.toml"), "[package");
    write(root.join("guardrail3-rs.toml"), "profile = [");

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed with fail-closed inputs");
    let results = g3rs_cargo_filetree_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CARGO-FILETREE-10"
                && result.title() == "declared workspace member missing Cargo.toml"
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CARGO-FILETREE-14"
                && result.file() == Some("crates/api/Cargo.toml")
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CARGO-FILETREE-14"
                && result.file() == Some("guardrail3-rs.toml")
        }),
        "{results:#?}"
    );
}

#[test]
fn filetree_pipeline_returns_exact_clean_inventory_results() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"
        "#,
    );
    write(
        root.join("crates/api/Cargo.toml"),
        r#"
            [package]
            name = "api"
            edition = "2024"
        "#,
    );
    write(root.join("guardrail3-rs.toml"), "profile = \"library\"\n");

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed for clean workspace");
    let results = g3rs_cargo_filetree_checks::check(&input);

    assert_eq!(results.len(), 2, "{results:#?}");
    assert_eq!(results[0].id(), "RS-CARGO-FILETREE-10");
    assert_eq!(results[0].title(), "all declared workspace members have Cargo.toml");
    assert!(results[0].inventory());
    assert_eq!(results[1].id(), "RS-CARGO-FILETREE-14");
    assert_eq!(results[1].title(), "cargo-family inputs parsed cleanly");
    assert!(results[1].inventory());
}

#[test]
fn filetree_pipeline_reports_malformed_workspace_members_without_missing_member_reclassification() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
            [workspace]
            members = "crates/api"
            resolver = "2"
        "#,
    );
    write(
        root.join("crates/api/Cargo.toml"),
        r#"
            [package]
            name = "api"
            edition = "2024"
        "#,
    );

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should degrade malformed workspace members to input failures");
    let results = g3rs_cargo_filetree_checks::check(&input);

    assert_eq!(results.len(), 2, "{results:#?}");
    assert!(results.iter().all(|result| result.id() == "RS-CARGO-FILETREE-14"));
    assert!(results.iter().all(|result| result.file() == Some("Cargo.toml")));
    assert!(results
        .iter()
        .all(|result| result.title() == "failed to read Cargo configuration"));
}

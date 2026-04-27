use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_cargo_ingestion_assertions::run as assertions;
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
            rule = "g3rs-cargo/approved-allow-inventory"
            file = "Cargo.toml"
            selector = "clippy:module_name_repetitions"
            reason = "Temporary lint suppression while API cleanup lands."
        "#,
    );

    let input =
        crate::run::ingest_for_config_checks(&crawl(root)).expect("ingestion should succeed");
    let results = g3rs_cargo_config_checks::check(&input);

    assertions::assert_config_pipeline_old_app_allow_inventory_and_member_rules(&results);
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

    let input =
        crate::run::ingest_for_config_checks(&crawl(root)).expect("ingestion should succeed");
    let results = g3rs_cargo_config_checks::check(&input);

    assertions::assert_config_pipeline_stands_down_allow_rules_when_guardrail3_rs_is_invalid(
        &results,
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
    write(
        root.join("guardrail3.toml"),
        "[profile]\nname = \"library\"\n",
    );

    let input = crate::run::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed with fail-closed inputs");
    let results = g3rs_cargo_filetree_checks::check(&input);

    assertions::assert_filetree_pipeline_reports_guardrail3_rs_parse_failures_and_ignores_legacy_guardrail3_toml(&results);
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

    let input = crate::run::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed with fail-closed inputs");
    let results = g3rs_cargo_filetree_checks::check(&input);

    assertions::assert_filetree_pipeline_reports_missing_member_and_input_failures(&results);
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

    let input = crate::run::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed for clean workspace");
    let results = g3rs_cargo_filetree_checks::check(&input);

    assertions::assert_filetree_pipeline_returns_exact_clean_inventory_results(&results);
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

    let input = crate::run::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should degrade malformed workspace members to input failures");
    let results = g3rs_cargo_filetree_checks::check(&input);

    assertions::assert_filetree_pipeline_reports_malformed_workspace_members_without_missing_member_reclassification(&results);
}

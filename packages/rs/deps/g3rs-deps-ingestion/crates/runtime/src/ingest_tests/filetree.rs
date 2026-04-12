use g3rs_workspace_crawl::crawl;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::run::ingest_for_file_tree_checks;

use super::{temp_workspace, write_file};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Finding {
    id: String,
    severity: G3Severity,
    title: String,
    message: String,
    file: Option<String>,
    inventory: bool,
}

fn findings(results: &[G3CheckResult]) -> Vec<Finding> {
    let mut findings = results
        .iter()
        .map(|result| Finding {
            id: result.id().to_owned(),
            severity: result.severity(),
            title: result.title().to_owned(),
            message: result.message().to_owned(),
            file: result.file().map(str::to_owned),
            inventory: result.inventory(),
        })
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| {
        (
            left.id.as_str(),
            format!("{:?}", left.severity),
            left.title.as_str(),
            left.message.as_str(),
            left.file.as_deref(),
            left.inventory,
        )
            .cmp(&(
                right.id.as_str(),
                format!("{:?}", right.severity),
                right.title.as_str(),
                right.message.as_str(),
                right.file.as_deref(),
                right.inventory,
            ))
    });
    findings
}

#[test]
fn filetree_entrypoint_ingests_root_lockfile_surface() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "service"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"
        "#,
    );
    write_file(workspace.path(), "Cargo.lock", "# lockfile\n");
    write_file(workspace.path(), ".gitignore", "target/\n");

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let input =
        ingest_for_file_tree_checks(&crawl).expect("deps filetree ingestion should succeed");

    assert_eq!(input.cargo_lock_rel_path, "Cargo.lock");
    assert!(input.cargo_lock_exists);
    assert!(!input.cargo_lock_ignored);
}

#[test]
fn pipeline_reports_missing_lockfile_as_error_for_service_profile() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "service"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"
        "#,
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let input = ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deps_filetree_checks::check(&input);

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-DEPS-FILETREE-09".to_owned(),
                severity: G3Severity::Error,
                title: "Cargo.lock missing".to_owned(),
                message:
                    "`Cargo.lock` is missing. Run `cargo generate-lockfile` and commit the result."
                        .to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-DEPS-FILETREE-10".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock tracked by git".to_owned(),
                message: "No relevant `.gitignore` masks `Cargo.lock` at the workspace root."
                    .to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
        ]
    );
}

#[test]
fn pipeline_reports_missing_lockfile_as_info_for_library_profile() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "library"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"
        "#,
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let input = ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deps_filetree_checks::check(&input);

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-DEPS-FILETREE-09".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock missing".to_owned(),
                message: "Library-profile workspace is missing `Cargo.lock`.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-DEPS-FILETREE-10".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock tracked by git".to_owned(),
                message: "No relevant `.gitignore` masks `Cargo.lock` at the workspace root."
                    .to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
        ]
    );
}

#[test]
fn pipeline_reports_ignored_lockfile() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "library"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"
        "#,
    );
    write_file(workspace.path(), "Cargo.lock", "# lockfile\n");
    write_file(workspace.path(), ".gitignore", "Cargo.lock\n");

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let input = ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deps_filetree_checks::check(&input);

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-DEPS-FILETREE-09".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock committed".to_owned(),
                message: "Workspace root has `Cargo.lock` committed.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-DEPS-FILETREE-10".to_owned(),
                severity: G3Severity::Error,
                title: "Cargo.lock ignored in gitignore".to_owned(),
                message: "`.gitignore` ignores `Cargo.lock`. Remove the line ignoring `Cargo.lock` from this `.gitignore`.".to_owned(),
                file: Some(".gitignore".to_owned()),
                inventory: false,
            },
        ]
    );
}

#[test]
fn pipeline_respects_unignore_after_ignore() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "service"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"
        "#,
    );
    write_file(workspace.path(), "Cargo.lock", "# lockfile\n");
    write_file(workspace.path(), ".gitignore", "Cargo.lock\n!Cargo.lock\n");

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let input = ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deps_filetree_checks::check(&input);

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-DEPS-FILETREE-09".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock committed".to_owned(),
                message: "Workspace root has `Cargo.lock` committed.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-DEPS-FILETREE-10".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock tracked by git".to_owned(),
                message: "No relevant `.gitignore` masks `Cargo.lock` at the workspace root."
                    .to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
        ]
    );
}

#[test]
fn pipeline_uses_last_gitignore_match() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "service"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"
        "#,
    );
    write_file(workspace.path(), "Cargo.lock", "# lockfile\n");
    write_file(workspace.path(), ".gitignore", "!Cargo.lock\nCargo.lock\n");

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let input = ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deps_filetree_checks::check(&input);

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-DEPS-FILETREE-09".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock committed".to_owned(),
                message: "Workspace root has `Cargo.lock` committed.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-DEPS-FILETREE-10".to_owned(),
                severity: G3Severity::Error,
                title: "Cargo.lock ignored in gitignore".to_owned(),
                message: "`.gitignore` ignores `Cargo.lock`. Remove the line ignoring `Cargo.lock` from this `.gitignore`.".to_owned(),
                file: Some(".gitignore".to_owned()),
                inventory: false,
            },
        ]
    );
}

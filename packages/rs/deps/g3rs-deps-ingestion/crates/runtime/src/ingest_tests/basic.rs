use g3rs_workspace_crawl::crawl;

use crate::run::{IngestionError, ingest_for_config_checks, ingest_for_source_checks};

use super::{temp_workspace, write_file};

#[test]
fn missing_guardrail_rs_file_fails_ingestion() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/*\"]\n",
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err = ingest_for_config_checks(&crawl).expect_err("missing guardrail3-rs.toml should fail");
    assert!(matches!(err, IngestionError::Guardrail3RsTomlNotFound));
}

#[test]
fn unreadable_root_guardrail_rs_file_fails_ingestion() {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

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
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    let guardrail_rs = workspace.path().join("guardrail3-rs.toml");
    let mut permissions = fs::metadata(&guardrail_rs)
        .expect("metadata should be readable")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&guardrail_rs, permissions).expect("chmod should succeed");

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err =
        ingest_for_config_checks(&crawl).expect_err("unreadable guardrail3-rs.toml should fail");
    assert!(matches!(err, IngestionError::Unreadable { .. }));
}

#[test]
fn malformed_root_guardrail_rs_file_fails_ingestion() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(workspace.path(), "guardrail3-rs.toml", "profile = [");
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err =
        ingest_for_config_checks(&crawl).expect_err("malformed guardrail3-rs.toml should fail");
    assert!(matches!(err, IngestionError::ParseFailed { .. }));
}

#[test]
fn source_entrypoint_stays_stubbed() {
    let workspace = temp_workspace();
    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");

    assert!(matches!(
        ingest_for_source_checks(&crawl),
        Err(IngestionError::SourceIngestionNotImplemented)
    ));
}

#[test]
fn unreadable_root_cargo_toml_fails_ingestion() {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

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
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    let cargo_toml = workspace.path().join("Cargo.toml");
    let mut permissions = fs::metadata(&cargo_toml)
        .expect("metadata should be readable")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&cargo_toml, permissions).expect("chmod should succeed");

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err = ingest_for_config_checks(&crawl).expect_err("unreadable Cargo.toml should fail");
    assert!(matches!(err, IngestionError::Unreadable { .. }));
}

#[test]
fn malformed_root_cargo_toml_fails_ingestion() {
    let workspace = temp_workspace();
    write_file(workspace.path(), "Cargo.toml", "[workspace\nmembers = [");
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "library"
        "#,
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err = ingest_for_config_checks(&crawl).expect_err("malformed Cargo.toml should fail");
    assert!(matches!(err, IngestionError::ParseFailed { .. }));
}

#[test]
fn unreadable_member_manifest_fails_ingestion() {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

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
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    let member_cargo = workspace.path().join("packages/core/Cargo.toml");
    let mut permissions = fs::metadata(&member_cargo)
        .expect("metadata should be readable")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&member_cargo, permissions).expect("chmod should succeed");

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err =
        ingest_for_config_checks(&crawl).expect_err("unreadable member Cargo.toml should fail");
    assert!(matches!(err, IngestionError::Unreadable { .. }));
}

#[test]
fn unknown_guardrail_key_is_ignored_when_owned_fields_are_valid() {
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
            allowed_deps = ["serde"]
            allowd_deps = ["reqwest"]
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"

            [dependencies]
            serde = "1"
        "#,
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let inputs = ingest_for_config_checks(&crawl).expect("unknown foreign key should be ignored");
    assert_eq!(inputs.len(), 1);
    assert!(inputs[0].allowlist_present);
    assert_eq!(inputs[0].allowed_deps, vec!["serde".to_owned()]);
}

#[test]
fn missing_declared_workspace_member_fails_ingestion() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core", "packages/missing"]
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
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err = ingest_for_config_checks(&crawl)
        .expect_err("missing declared member should fail ingestion");
    assert!(matches!(
        err,
        IngestionError::NormalizationFailed { reason, .. }
            if reason.contains("packages/missing")
    ));
}

#[test]
fn empty_allowlist_stays_present_while_missing_allowlist_stays_absent() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [package]
            name = "root-crate"
            version = "0.1.0"

            [workspace]
            members = ["packages/*"]
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "library"
            allowed_deps = []
        "#,
    );
    let workspace_crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let inputs = ingest_for_config_checks(&workspace_crawl).expect("empty allowlist should ingest");
    assert!(
        inputs.iter().all(|input| input.allowlist_present),
        "empty allowlist should still be marked as present: {inputs:#?}"
    );
    assert!(
        inputs.iter().all(|input| input.allowed_deps.is_empty()),
        "empty allowlist should stay empty: {inputs:#?}"
    );

    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "library"
        "#,
    );
    let workspace_crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let inputs =
        ingest_for_config_checks(&workspace_crawl).expect("missing allowlist should ingest");
    assert!(
        inputs.iter().all(|input| !input.allowlist_present),
        "missing allowlist should stay absent: {inputs:#?}"
    );
    assert!(
        inputs.iter().all(|input| input.allowed_deps.is_empty()),
        "missing allowlist should still have an empty vector payload: {inputs:#?}"
    );
}

#[test]
fn empty_allowed_dep_entry_fails_ingestion() {
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
            allowed_deps = [""]
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
    let err = ingest_for_config_checks(&crawl).expect_err("empty allowed_deps entry should fail");
    assert!(matches!(
        err,
        IngestionError::NormalizationFailed { reason, .. }
            if reason.contains("must not contain empty dependency names")
    ));
}

#[test]
fn in_workspace_non_member_path_dependency_fails_ingestion() {
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

            [dependencies]
            helper = { path = "../../vendor/helper" }
        "#,
    );
    write_file(
        workspace.path(),
        "vendor/helper/Cargo.toml",
        "[package]\nname = \"helper\"\nversion = \"0.1.0\"\n",
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err =
        ingest_for_config_checks(&crawl).expect_err("in-workspace non-member path should fail");
    assert!(matches!(
        err,
        IngestionError::NormalizationFailed { reason, .. }
            if reason.contains("in-workspace non-member")
    ));
}

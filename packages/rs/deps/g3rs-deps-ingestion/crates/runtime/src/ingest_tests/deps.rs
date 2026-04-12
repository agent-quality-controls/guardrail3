use g3rs_deps_types::G3RsDepsDependencySection;
use g3rs_deps_types::G3RsDepsConfigInputScope;
use g3rs_workspace_crawl::crawl;
use guardrail3_rs_toml_parser::RustProfile;

use crate::run::ingest_for_config_checks;

use super::{temp_workspace, write_file};

#[test]
fn ingests_member_crates_into_normalized_dependency_inputs() {
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
            exclude = ["packages/excluded"]

            [workspace.dependencies]
            serde = "1"
            vendored_reqwest = { package = "reqwest", path = "/opt/vendor/reqwest" }
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "library"
            allowed_deps = ["serde", "bytes"]
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
            serde = { workspace = true }
            vendored_reqwest = { workspace = true }
            bytes_alias = { package = "bytes", version = "1" }
            support = { path = "../support" }
            outside = { package = "reqwest", path = "../../../vendor/reqwest" }

            [build-dependencies]
            bindgen_alias = { package = "bindgen", version = "0.70" }

            [dev-dependencies]
            proptest = "1"

            [target.'cfg(unix)'.dependencies]
            tempfile = "3"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/support/Cargo.toml",
        r#"
            [package]
            name = "support"
            version = "0.1.0"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/excluded/Cargo.toml",
        r#"
            [package]
            name = "excluded"
            version = "0.1.0"
        "#,
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let inputs = ingest_for_config_checks(&crawl).expect("deps ingestion should succeed");
    let crate_inputs = inputs
        .iter()
        .filter(|input| input.scope == G3RsDepsConfigInputScope::CratePolicy)
        .collect::<Vec<_>>();
    assert_eq!(
        crate_inputs.len(),
        3,
        "root + core + support should each get one input"
    );
    assert!(
        crate_inputs.iter().any(|input| input.crate_name == "root-crate"),
        "hybrid workspace root should be ingested: {inputs:#?}"
    );
    assert!(
        crate_inputs.iter().any(|input| input.crate_name == "support"),
        "dependency-free member should still be ingested: {inputs:#?}"
    );
    assert!(
        crate_inputs.iter().all(|input| input.crate_name != "excluded"),
        "excluded workspace members should not be ingested: {inputs:#?}"
    );

    let core_input = crate_inputs
        .iter()
        .find(|input| input.crate_name == "core")
        .expect("core input should exist");

    assert_eq!(core_input.profile, Some(RustProfile::Library));
    assert!(core_input.allowlist_present);
    assert_eq!(
        core_input.allowed_deps,
        vec!["serde".to_owned(), "bytes".to_owned()]
    );
    assert_eq!(
        core_input
            .dependencies
            .iter()
            .map(|entry| (
                entry.package_name.as_str(),
                entry.section,
                entry.table_label.as_str()
            ))
            .collect::<Vec<_>>(),
        vec![
            (
                "bytes",
                G3RsDepsDependencySection::Dependencies,
                "[dependencies]"
            ),
            (
                "reqwest",
                G3RsDepsDependencySection::Dependencies,
                "[dependencies]"
            ),
            (
                "serde",
                G3RsDepsDependencySection::Dependencies,
                "[dependencies]"
            ),
            (
                "reqwest",
                G3RsDepsDependencySection::Dependencies,
                "[dependencies]"
            ),
            (
                "bindgen",
                G3RsDepsDependencySection::BuildDependencies,
                "[build-dependencies]"
            ),
            (
                "proptest",
                G3RsDepsDependencySection::DevDependencies,
                "[dev-dependencies]"
            ),
            (
                "tempfile",
                G3RsDepsDependencySection::Dependencies,
                "[target.'cfg(unix)'.dependencies]"
            ),
        ]
    );
}

#[test]
fn undefined_workspace_dependency_fails_ingestion() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/*"]
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
            serde = { workspace = true }
        "#,
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err =
        ingest_for_config_checks(&crawl).expect_err("undefined workspace dependency should fail");
    assert!(matches!(
        err,
        crate::run::IngestionError::NormalizationFailed { reason, .. }
            if reason.contains("workspace dependency `serde` was requested but not defined")
    ));
}

#[test]
fn hybrid_workspace_root_path_dependency_to_root_stays_internal() {
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
        "guardrail3-rs.toml",
        r#"
            profile = "library"
            allowed_deps = ["serde"]
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
            root_crate = { path = "../.." }
            serde = "1"
        "#,
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let inputs =
        ingest_for_config_checks(&crawl).expect("hybrid root path dependency should stay internal");

    let core_input = inputs
        .iter()
        .find(|input| input.crate_name == "core")
        .expect("core input should exist");
    assert_eq!(
        core_input
            .dependencies
            .iter()
            .map(|entry| entry.package_name.as_str())
            .collect::<Vec<_>>(),
        vec!["serde"],
        "path dependency back to the hybrid root should not be treated as external: {core_input:#?}"
    );
}

#[test]
fn absolute_path_dependency_to_workspace_member_stays_internal() {
    let workspace = temp_workspace();
    let support_dir = workspace.path().join("packages/support");
    let support_path = support_dir.to_string_lossy().into_owned();

    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/*"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "library"
            allowed_deps = ["serde"]
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        &format!(
            r#"
                [package]
                name = "core"
                version = "0.1.0"

                [dependencies]
                support = {{ path = "{support_path}" }}
                serde = "1"
            "#
        ),
    );
    write_file(
        workspace.path(),
        "packages/support/Cargo.toml",
        r#"
            [package]
            name = "support"
            version = "0.1.0"
        "#,
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let inputs = ingest_for_config_checks(&crawl)
        .expect("absolute path to a workspace member should stay internal");

    let core_input = inputs
        .iter()
        .find(|input| input.crate_name == "core")
        .expect("core input should exist");
    assert_eq!(
        core_input
            .dependencies
            .iter()
            .map(|entry| entry.package_name.as_str())
            .collect::<Vec<_>>(),
        vec!["serde"],
        "absolute path dependency to a workspace member should not be treated as external: {core_input:#?}"
    );
}

#[test]
fn absolute_path_dependency_inside_workspace_non_member_fails_ingestion() {
    let workspace = temp_workspace();
    let helper_dir = workspace.path().join("vendor/helper");
    let helper_path = helper_dir.to_string_lossy().into_owned();

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
        &format!(
            r#"
                [package]
                name = "core"
                version = "0.1.0"

                [dependencies]
                helper = {{ path = "{helper_path}" }}
            "#
        ),
    );
    write_file(
        workspace.path(),
        "vendor/helper/Cargo.toml",
        r#"
            [package]
            name = "helper"
            version = "0.1.0"
        "#,
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err = ingest_for_config_checks(&crawl)
        .expect_err("absolute in-workspace non-member path should fail");
    assert!(matches!(
        err,
        crate::run::IngestionError::NormalizationFailed { reason, .. }
            if reason.contains("in-workspace non-member")
    ));
}

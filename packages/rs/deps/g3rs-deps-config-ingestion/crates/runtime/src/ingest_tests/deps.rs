use g3rs_deps_types::G3RsDepsDependencySection;
use g3rs_workspace_crawl::crawl;
use guardrail3_rs_toml_parser::RustProfile;

use crate::run::ingest_config;

use super::{temp_workspace, write_file};

#[test]
fn ingests_member_crates_into_normalized_dependency_inputs() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/*"]

            [workspace.dependencies]
            serde = "1"
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
            bytes_alias = { package = "bytes", version = "1" }
            support = { path = "../support" }
            outside = { package = "reqwest", path = "../../../vendor/reqwest" }

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

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let inputs = ingest_config(&crawl).expect("deps ingestion should succeed");

    let core_input = inputs
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
                "tempfile",
                G3RsDepsDependencySection::Dependencies,
                "[target.'cfg(unix)'.dependencies]"
            ),
        ]
    );
}

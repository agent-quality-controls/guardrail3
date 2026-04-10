use g3rs_test_ingestion_types::{
    G3RsTestSourceChecksInput, G3RsTestConfigChecksInput, G3RsTestFileTreeChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

pub use g3rs_test_ingestion_types::G3RsTestIngestionError as IngestionError;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsTestConfigChecksInput>, IngestionError> {
    ingest_for_config_checks_with_tool_state(crawl, cargo_mutants_installed())
}

pub(crate) fn ingest_for_config_checks_with_tool_state(
    crawl: &G3RsWorkspaceCrawl,
    cargo_mutants_installed: bool,
) -> Result<Vec<G3RsTestConfigChecksInput>, IngestionError> {
    let discovery = crate::roots::discover(crawl)?;

    discovery
        .roots
        .iter()
        .map(|root| {
            let activation =
                crate::activation::summarize_root(crawl, root, discovery.workspace_manifest.as_ref())?;
            let hook_state = crate::hooks::collect_mutation_hook_state(crawl, &discovery, root)?;
            let nextest_rel_path = crate::roots::join_under_root(&root.root_rel_dir, ".config/nextest.toml");
            let mutants_rel_path = crate::roots::join_under_root(&root.root_rel_dir, ".cargo/mutants.toml");
            let nextest = parse_optional_nextest(crawl, &nextest_rel_path)?;
            let (mutants_exists, mutants) = parse_optional_mutants(crawl, &mutants_rel_path)?;

            Ok(G3RsTestConfigChecksInput {
                root_rel_dir: root.root_rel_dir.clone(),
                cargo_rel_path: root.cargo_rel_path.clone(),
                mutants_rel_path,
                nextest_rel_path,
                cargo: root.cargo.clone(),
                nextest,
                mutants,
                has_tests: activation.has_tests,
                has_tokio_tests: activation.has_tokio_tests,
                tokio_dependency_present: crate::activation::has_tokio_dependency(
                    &root.cargo,
                    discovery.workspace_manifest.as_ref(),
                ),
                cargo_mutants_installed,
                mutation_hook_active: hook_state.active,
                mutation_hook_files: hook_state.files,
                mutants_exists,
            })
        })
        .collect()
}

pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsTestSourceChecksInput>, IngestionError> {
    let discovery = crate::roots::discover(crawl)?;

    discovery
        .roots
        .iter()
        .map(|root| {
            let components = crate::components::collect_components(crawl, root)?;
            let files = crate::components::collect_ast_files(crawl, root, &components)?;
            Ok(G3RsTestSourceChecksInput {
                root_rel_dir: root.root_rel_dir.clone(),
                cargo_rel_path: root.cargo_rel_path.clone(),
                files,
                components: crate::components::public_component_facts(&components),
            })
        })
        .collect()
}

pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsTestFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}

fn parse_optional_nextest(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
) -> Result<Option<nextest_toml_parser::NextestToml>, IngestionError> {
    let Some(entry) = crawl.entry(rel_path) else {
        return Ok(None);
    };
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let content = crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| IngestionError::Unreadable {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;
    nextest_toml_parser::parse(&content).map(Some).map_err(|err| IngestionError::ParseFailed {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })
}

fn parse_optional_mutants(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
) -> Result<(bool, Option<mutants_toml_parser::MutantsToml>), IngestionError> {
    let Some(entry) = crawl.entry(rel_path) else {
        return Ok((false, None));
    };
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let content = crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| IngestionError::Unreadable {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;
    let parsed = mutants_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;
    Ok((true, Some(parsed)))
}

fn cargo_mutants_installed() -> bool {
    std::process::Command::new("cargo-mutants")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

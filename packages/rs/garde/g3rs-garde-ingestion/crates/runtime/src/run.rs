/// Public ingestion entry point.
use g3rs_garde_source_checks_types::{G3RsSourceFile, G3RsGardeSourceChecksInput};
use cargo_toml_parser::CargoToml;
use g3rs_garde_types::{
    G3RsGardeClippyInput, G3RsGardeConfigChecksInput, G3RsGardeFileTreeChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsGardeIngestionError` so the facade can reach it.
pub use g3rs_garde_ingestion_types::G3RsGardeIngestionError as IngestionError;

/// Ingest garde config from a workspace crawl into a config checks input.
///
/// Cargo.toml is required. Clippy config is optional — if absent,
/// the clippy state will be `Missing` in the result and the config lane
/// will emit its own "cannot verify" warnings when garde is present.
///
/// # Errors
///
/// Returns an error if Cargo.toml is missing, unreadable, or unparseable.
pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsGardeConfigChecksInput, IngestionError> {
    // 1. Select and parse Cargo.toml (required)
    let cargo_entry = crate::select::select_cargo_toml(crawl)
        .ok_or(IngestionError::CargoTomlNotFound)?;

    if !cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let cargo = crate::parse::parse_cargo_toml(&cargo_entry.path.abs_path)?;

    // 2. Select and classify clippy config (optional)
    let clippy_input = if let Some(entry) = crate::select::select_clippy_toml(crawl) {
        if !entry.readable {
            G3RsGardeClippyInput::Invalid {
                rel_path: entry.path.rel_path.clone(),
                message: format!(
                    "Failed to read `{}` for garde clippy-ban validation: file is not readable",
                    entry.path.rel_path
                ),
            }
        } else {
            match crate::parse::parse_clippy_toml(&entry.path.abs_path) {
                Ok(parsed) => G3RsGardeClippyInput::Parsed {
                    rel_path: entry.path.rel_path.clone(),
                    clippy: parsed,
                },
                Err(IngestionError::Unreadable { reason, .. }) => G3RsGardeClippyInput::Invalid {
                    rel_path: entry.path.rel_path.clone(),
                    message: format!(
                        "Failed to read `{}` for garde clippy-ban validation: {reason}",
                        entry.path.rel_path
                    ),
                },
                Err(IngestionError::ParseFailed { reason, .. }) => G3RsGardeClippyInput::Invalid {
                    rel_path: entry.path.rel_path.clone(),
                    message: format!(
                        "Failed to parse `{}` for garde clippy-ban validation: {reason}",
                        entry.path.rel_path
                    ),
                },
                Err(other) => return Err(other),
            }
        }
    } else {
        G3RsGardeClippyInput::Missing
    };

    Ok(crate::ingest::assemble(cargo_entry.path.rel_path.clone(), cargo, clippy_input))
}

/// Ingest garde source input from a workspace crawl.
pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsGardeSourceChecksInput, IngestionError> {
    let cargo_entry = crate::select::select_cargo_toml(crawl)
        .ok_or(IngestionError::CargoTomlNotFound)?;
    if !cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let cargo = crate::parse::parse_cargo_toml(&cargo_entry.path.abs_path)?;
    let guardrail_entry =
        crate::select::select_guardrail_toml(crawl).ok_or(IngestionError::GuardrailTomlNotFound)?;
    let source_files = crate::select::select_ast_source_files(crawl)
        .into_iter()
        .map(|entry| G3RsSourceFile {
            rel_path: entry.path.rel_path.clone(),
            abs_path: entry.path.abs_path.clone(),
        })
        .collect::<Vec<_>>();

    Ok(G3RsGardeSourceChecksInput {
        garde_dependency_present: has_garde_dependency(&cargo),
        source_files,
        guardrail_toml: G3RsSourceFile {
            rel_path: guardrail_entry.path.rel_path.clone(),
            abs_path: guardrail_entry.path.abs_path.clone(),
        },
    })
}

fn has_garde_dependency(cargo: &CargoToml) -> bool {
    cargo.dependencies.contains_key("garde")
        || cargo
            .workspace
            .as_ref()
            .is_some_and(|workspace| workspace.dependencies.contains_key("garde"))
}

/// Stub file-tree ingestion entry point for the garde family.
pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsGardeFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}

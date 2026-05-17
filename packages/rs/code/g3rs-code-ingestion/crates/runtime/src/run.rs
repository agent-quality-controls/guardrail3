use cargo_toml_parser::parse;
use g3_workspace_crawl::G3WorkspaceCrawl;
use g3rs_code_types::{
    G3RsCodeConfigChecksInput, G3RsCodeFileTreeChecksInput, G3RsCodeSourceChecksInput,
    G3RsCodeStructuralCapRoot,
};

/// Re-export of `G3RsCodeIngestionError` so the facade can reach it.
pub use g3rs_code_ingestion_types::G3RsCodeIngestionError as IngestionError;

/// Implements this item.
///
/// # Errors
/// Returns an error when the underlying operation fails.
pub fn ingest_for_config_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3RsCodeConfigChecksInput, IngestionError> {
    crate::config_files::collect_config_files(crawl)
}

/// Ingest `code` source checks input from a workspace crawl.
///
/// List of `G3RsCodeSourceChecksInput` records produced by `ingest_for_source_checks`.
type SourceChecksInputs = Vec<G3RsCodeSourceChecksInput>;

/// # Errors
/// Returns an error when the underlying operation fails.
pub fn ingest_for_source_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<SourceChecksInputs, IngestionError> {
    crate::select::select_source_files(crawl)?
        .into_iter()
        .map(|selected| {
            if !selected.entry.readable {
                return Err(IngestionError::Unreadable {
                    path: selected.entry.path.abs_path.clone(),
                    reason: "file is not readable".to_owned(),
                });
            }

            let content =
                crate::fs::read_to_string(&selected.entry.path.abs_path).map_err(|err| {
                    IngestionError::Unreadable {
                        path: selected.entry.path.abs_path.clone(),
                        reason: err.to_string(),
                    }
                })?;
            Ok(crate::ingest::assemble(crate::ingest::AssembleInputs {
                rel_path: selected.entry.path.rel_path.clone(),
                content,
                is_test: selected.is_test,
                profile_name: selected.profile_name,
                is_library_root: selected.is_library_root,
                is_shared_crate: selected.is_shared_crate,
                waivers: selected.waivers,
            }))
        })
        .collect()
}

/// Implements this item.
///
/// # Errors
/// Returns an error when the underlying operation fails.
/// Ingest `code` file-tree checks input from a workspace crawl.
pub fn ingest_for_file_tree_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3RsCodeFileTreeChecksInput, IngestionError> {
    Ok(G3RsCodeFileTreeChecksInput {
        roots: selected_package_roots(crawl)?,
    })
}

/// List of `G3RsCodeStructuralCapRoot` records, one per package root selected for the cap-roots inventory.
type PackageRoots = Vec<G3RsCodeStructuralCapRoot>;

/// Implements `selected package roots`.
fn selected_package_roots(crawl: &G3WorkspaceCrawl) -> Result<PackageRoots, IngestionError> {
    let Some(root_cargo_entry) = g3_workspace_crawl::root_file(crawl, "Cargo.toml") else {
        return Err(IngestionError::ParseFailed {
            path: crawl.root_abs_path.join("Cargo.toml"),
            reason: "root Cargo.toml missing from crawl".to_owned(),
        });
    };

    if !root_cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: root_cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let root_content =
        crate::fs::read_to_string(&root_cargo_entry.path.abs_path).map_err(|err| {
            IngestionError::Unreadable {
                path: root_cargo_entry.path.abs_path.clone(),
                reason: err.to_string(),
            }
        })?;
    let root_manifest = parse(&root_content).map_err(|err| IngestionError::ParseFailed {
        path: root_cargo_entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;

    let mut roots = crate::config_scope::owned_root_dirs(crawl)?
        .into_iter()
        .filter(|rel_dir| !rel_dir.is_empty())
        .map(|rel_dir| G3RsCodeStructuralCapRoot {
            cargo_rel_path: join_rel(&rel_dir, "Cargo.toml"),
            root_rel_dir: rel_dir,
        })
        .collect::<Vec<_>>();

    if root_manifest.package.is_some() {
        roots.push(G3RsCodeStructuralCapRoot {
            root_rel_dir: String::new(),
            cargo_rel_path: "Cargo.toml".to_owned(),
        });
    }

    roots.sort_by(|left, right| left.root_rel_dir.cmp(&right.root_rel_dir));
    Ok(roots)
}

/// Implements `join rel`.
fn join_rel(dir: &str, child: &str) -> String {
    if dir.is_empty() {
        child.to_owned()
    } else {
        format!("{dir}/{child}")
    }
}

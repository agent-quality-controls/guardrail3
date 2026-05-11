use g3_workspace_crawl::G3WorkspaceCrawl;
use g3rs_clippy_types::{
    G3RsClippyCargoRootState, G3RsClippyConfigChecksInput, G3RsClippyFileTreeChecksInput,
    G3RsClippyRustPolicyState, G3RsClippyShadowedConfig,
};

pub use crate::error::G3RsClippyIngestionError as IngestionError;

/// Build the config-checks input snapshot for a workspace crawl.
///
/// # Errors
///
/// Returns an error when the preferred `clippy.toml` is missing, or when the
/// guardrail3-rs policy waivers fail to parse on a parsed policy state.
pub fn ingest_for_config_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3RsClippyConfigChecksInput, IngestionError> {
    let clippy_entry = crate::select::select_preferred_root_clippy_toml(crawl)
        .ok_or(IngestionError::ClippyTomlNotFound)?;

    let clippy = crate::parse::parse_clippy_state(&clippy_entry.path.abs_path);
    let (rust_policy, waivers) = match crate::select::select_root_guardrail3_rs_toml(crawl) {
        Some(policy_entry) => {
            let rust_policy = crate::parse::parse_rust_policy_state(
                &policy_entry.path.rel_path,
                &policy_entry.path.abs_path,
            );
            let waivers = match &rust_policy {
                G3RsClippyRustPolicyState::Parsed { .. } => {
                    crate::parse::parse_waivers(&policy_entry.path.abs_path)?
                }
                G3RsClippyRustPolicyState::Missing
                | G3RsClippyRustPolicyState::Unreadable { .. }
                | G3RsClippyRustPolicyState::ParseError { .. } => Vec::new(),
            };
            (rust_policy, waivers)
        }
        None => (G3RsClippyRustPolicyState::Missing, Vec::new()),
    };
    let cargo_root = crate::select::select_root_cargo_toml(crawl).map_or(
        G3RsClippyCargoRootState::Missing,
        |cargo_entry| {
            crate::parse::parse_cargo_root_state(
                &cargo_entry.path.rel_path,
                &cargo_entry.path.abs_path,
            )
        },
    );
    let cargo_workspace_members = collect_workspace_member_states(crawl, &cargo_root);
    let cargo_configs = crate::select::collect_root_cargo_config_overrides(crawl)
        .into_iter()
        .map(|config_entry| {
            crate::parse::parse_cargo_config_state(
                &config_entry.path.rel_path,
                &config_entry.path.abs_path,
            )
        })
        .collect();

    Ok(crate::ingest::assemble_config_input(
        clippy_entry.path.rel_path.clone(),
        clippy,
        rust_policy,
        cargo_root,
        cargo_workspace_members,
        cargo_configs,
        waivers,
    ))
}

/// Build the file-tree-checks input snapshot for a workspace crawl.
///
/// # Errors
///
/// This currently does not produce an error; the `Result` is retained to keep
/// the ingestion API uniform with other family ingestion entry points.
#[expect(
    clippy::unnecessary_wraps,
    reason = "uniform ingestion entry-point signature across the family; downstream callers consume `Result` from every ingest_for_* fn so collapsing to plain `T` here would diverge from the cross-package contract"
)]
pub fn ingest_for_file_tree_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3RsClippyFileTreeChecksInput, IngestionError> {
    let root_configs = crate::select::collect_root_clippy_tomls(crawl);
    let preferred = root_configs
        .first()
        .map(|entry| entry.path.rel_path.clone());
    let shadowed_same_root_configs =
        preferred
            .as_deref()
            .map_or_else(Vec::new, |preferred_rel_path| {
                root_configs
                    .into_iter()
                    .filter(|entry| entry.path.rel_path != preferred_rel_path)
                    .map(|entry| G3RsClippyShadowedConfig {
                        rel_path: entry.path.rel_path.clone(),
                        preferred_rel_path: preferred_rel_path.to_owned(),
                    })
                    .collect()
            });

    Ok(crate::ingest::assemble_filetree_input(
        preferred,
        shadowed_same_root_configs,
    ))
}

/// Resolve workspace-member Cargo state entries for the parsed cargo root.
fn collect_workspace_member_states(
    crawl: &G3WorkspaceCrawl,
    cargo_root: &G3RsClippyCargoRootState,
) -> Vec<g3rs_clippy_types::G3RsClippyCargoMemberState> {
    let G3RsClippyCargoRootState::Parsed { cargo, .. } = cargo_root else {
        return Vec::new();
    };
    if cargo_toml_parser::document::kind(cargo)
        != cargo_toml_parser::types::CargoTomlDocumentKind::WorkspaceRoot
    {
        return Vec::new();
    }
    crate::parse::collect_declared_member_rels(&crawl.root_abs_path, &cargo.raw)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|member_rel| {
            let rel_path = if member_rel.is_empty() {
                "Cargo.toml".to_owned()
            } else {
                format!("{member_rel}/Cargo.toml")
            };
            let member_entry = g3_workspace_crawl::entry(crawl, &rel_path)?;
            Some(crate::parse::parse_cargo_member_state(
                &member_rel,
                &member_entry.path.rel_path,
                &member_entry.path.abs_path,
            ))
        })
        .collect()
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;

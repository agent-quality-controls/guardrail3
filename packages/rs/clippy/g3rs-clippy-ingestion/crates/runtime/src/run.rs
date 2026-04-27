use g3rs_clippy_types::{
    G3RsClippyCargoRootState, G3RsClippyConfigChecksInput, G3RsClippyFileTreeChecksInput,
    G3RsClippyRustPolicyState, G3RsClippyShadowedConfig,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

pub use crate::error::G3RsClippyIngestionError as IngestionError;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsClippyConfigChecksInput, IngestionError> {
    let entry = crate::select::select_preferred_root_clippy_toml(crawl)
        .ok_or(IngestionError::ClippyTomlNotFound)?;

    let clippy = crate::parse::parse_clippy_state(&entry.path.abs_path);
    let (rust_policy, waivers) = match crate::select::select_root_guardrail3_rs_toml(crawl) {
        Some(entry) => {
            let rust_policy =
                crate::parse::parse_rust_policy_state(&entry.path.rel_path, &entry.path.abs_path);
            let waivers = match &rust_policy {
                G3RsClippyRustPolicyState::Parsed { .. } => {
                    crate::parse::parse_waivers(&entry.path.abs_path)?
                }
                G3RsClippyRustPolicyState::Missing
                | G3RsClippyRustPolicyState::Unreadable { .. }
                | G3RsClippyRustPolicyState::ParseError { .. } => Vec::new(),
            };
            (rust_policy, waivers)
        }
        None => (G3RsClippyRustPolicyState::Missing, Vec::new()),
    };
    let cargo_root = match crate::select::select_root_cargo_toml(crawl) {
        Some(entry) => {
            crate::parse::parse_cargo_root_state(&entry.path.rel_path, &entry.path.abs_path)
        }
        None => G3RsClippyCargoRootState::Missing,
    };
    let cargo_workspace_members = match &cargo_root {
        G3RsClippyCargoRootState::Parsed { cargo, .. }
            if cargo_toml_parser::document::kind(cargo)
                == cargo_toml_parser::types::CargoTomlDocumentKind::WorkspaceRoot =>
        {
            crate::parse::collect_declared_member_rels(&crawl.root_abs_path, &cargo.raw)
                .unwrap_or_default()
                .into_iter()
                .filter_map(|member_rel| {
                    let rel_path = if member_rel.is_empty() {
                        "Cargo.toml".to_owned()
                    } else {
                        format!("{member_rel}/Cargo.toml")
                    };
                    let entry = g3rs_workspace_crawl::entry(crawl, &rel_path)?;
                    Some(crate::parse::parse_cargo_member_state(
                        &member_rel,
                        &entry.path.rel_path,
                        &entry.path.abs_path,
                    ))
                })
                .collect()
        }
        G3RsClippyCargoRootState::Missing
        | G3RsClippyCargoRootState::Unreadable { .. }
        | G3RsClippyCargoRootState::ParseError { .. }
        | G3RsClippyCargoRootState::Parsed { .. } => Vec::new(),
    };
    let cargo_configs = crate::select::collect_root_cargo_config_overrides(crawl)
        .into_iter()
        .map(|entry| {
            crate::parse::parse_cargo_config_state(&entry.path.rel_path, &entry.path.abs_path)
        })
        .collect();

    Ok(crate::ingest::assemble_config_input(
        entry.path.rel_path.clone(),
        clippy,
        rust_policy,
        cargo_root,
        cargo_workspace_members,
        cargo_configs,
        waivers,
    ))
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsClippyFileTreeChecksInput, IngestionError> {
    let root_configs = crate::select::collect_root_clippy_tomls(crawl);
    let preferred = root_configs
        .first()
        .map(|entry| entry.path.rel_path.clone());
    let shadowed_same_root_configs = match preferred.as_deref() {
        Some(preferred_rel_path) => root_configs
            .into_iter()
            .filter(|entry| entry.path.rel_path != preferred_rel_path)
            .map(|entry| G3RsClippyShadowedConfig {
                rel_path: entry.path.rel_path.clone(),
                preferred_rel_path: preferred_rel_path.to_owned(),
            })
            .collect(),
        None => Vec::new(),
    };

    Ok(crate::ingest::assemble_filetree_input(
        preferred,
        shadowed_same_root_configs,
    ))
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;

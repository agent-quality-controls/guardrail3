use g3rs_clippy_types::{
    G3RsClippyConfigChecksInput, G3RsClippyFileTreeChecksInput, G3RsClippyPolicyContextState,
    G3RsClippyShadowedConfig,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

pub use g3rs_clippy_ingestion_types::G3RsClippyIngestionError as IngestionError;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsClippyConfigChecksInput, IngestionError> {
    let entry = crate::select::select_preferred_root_clippy_toml(crawl)
        .ok_or(IngestionError::ClippyTomlNotFound)?;

    let clippy = crate::parse::parse_clippy_state(&entry.path.abs_path);
    let policy_context = match crate::select::select_root_guardrail_toml(crawl) {
        Some(entry) => crate::parse::parse_guardrail_policy_state(
            &entry.path.rel_path,
            &entry.path.abs_path,
        ),
        None => G3RsClippyPolicyContextState::Missing,
    };
    let profile_name = match &policy_context {
        G3RsClippyPolicyContextState::Parsed { profile_name, .. } => profile_name.as_deref(),
        G3RsClippyPolicyContextState::Missing
        | G3RsClippyPolicyContextState::Unreadable { .. }
        | G3RsClippyPolicyContextState::ParseError { .. } => None,
    };
    let published_library_policy = match crate::select::select_root_cargo_toml(crawl) {
        Some(entry) => crate::parse::compute_published_library_policy(
            &crawl.root_abs_path,
            &entry.path.abs_path,
            profile_name,
        ),
        None => false,
    };
    let cargo_config_overrides = crate::select::collect_root_cargo_config_overrides(crawl)
        .into_iter()
        .filter_map(|entry| {
            crate::parse::parse_cargo_override(&entry.path.rel_path, &entry.path.abs_path)
        })
        .collect();

    Ok(crate::ingest::assemble_config_input(
        entry.path.rel_path.clone(),
        clippy,
        policy_context,
        published_library_policy,
        cargo_config_overrides,
    ))
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsClippyFileTreeChecksInput, IngestionError> {
    let root_configs = crate::select::collect_root_clippy_tomls(crawl);
    let preferred = root_configs.first().map(|entry| entry.path.rel_path.clone());
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

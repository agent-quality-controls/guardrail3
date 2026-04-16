use g3rs_code_ingestion_types::G3RsCodeIngestionError as IngestionError;
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind};

/// Selected Rust source file metadata ready to be read and mapped.
pub(crate) struct SelectedCodeSourceFile<'a> {
    /// Underlying crawl entry.
    pub(crate) entry: &'a G3RsWorkspaceEntry,
    /// Whether the file belongs to test-owned code.
    pub(crate) is_test: bool,
    /// Optional pre-resolved policy profile.
    pub(crate) profile_name: Option<String>,
    /// Whether this file is the exact library root source.
    pub(crate) is_library_root: bool,
    /// Whether the owning crate is explicitly marked shared.
    pub(crate) is_shared_crate: bool,
    /// Package-local waivers that apply to source checks.
    pub(crate) waivers: Vec<g3rs_code_types::G3RsCodeWaiver>,
}

/// Select all owned Rust source files for the `code` source lane.
pub(crate) fn select_source_files(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<SelectedCodeSourceFile<'_>>, IngestionError> {
    let source_entries = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.ends_with(".rs"))
        .filter(|entry| !crate::classify::is_fixture_path(entry.path.rel_path.as_str()))
        .collect::<Vec<_>>();

    let classifier = crate::classify::CargoTargetClassifier::build(
        crawl,
        &source_entries
            .iter()
            .map(|entry| entry.path.rel_path.clone())
            .collect::<Vec<_>>(),
    )?;

    Ok(source_entries
        .into_iter()
        .map(|entry| {
            let is_test = crate::classify::is_test_root_path(entry.path.rel_path.as_str());
            let profile = classifier.classify(entry.path.rel_path.as_str(), is_test);

            SelectedCodeSourceFile {
                entry,
                is_test,
                profile_name: profile.profile_name,
                is_library_root: profile.is_library_root,
                is_shared_crate: profile.is_shared_crate,
                waivers: profile.waivers,
            }
        })
        .collect())
}

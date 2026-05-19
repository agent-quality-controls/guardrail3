use g3_workspace_crawl::{G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind};
use g3rs_code_ingestion_types::G3RsCodeIngestionError as IngestionError;

/// Selected Rust source file metadata ready to be read and mapped.
pub(crate) struct SelectedCodeSourceFile<'a> {
    /// Underlying crawl entry.
    pub(crate) entry: &'a G3WorkspaceEntry,
    /// Whether the file belongs to test-owned code.
    pub(crate) is_test: bool,
    /// Optional pre-resolved policy profile.
    pub(crate) profile_name: Option<String>,
    /// Whether this file is the exact library root source.
    pub(crate) is_library_root: bool,
    /// Whether the owning crate is explicitly marked shared.
    pub(crate) is_shared_crate: bool,
}

/// List of selected Rust source files (with their classifier metadata) for the `code` source lane.
pub(crate) type SelectedCodeSourceFiles<'a> = Vec<SelectedCodeSourceFile<'a>>;

/// Select all owned Rust source files for the `code` source lane.
pub(crate) fn select_source_files(
    crawl: &G3WorkspaceCrawl,
) -> Result<SelectedCodeSourceFiles<'_>, IngestionError> {
    let source_entries = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3WorkspaceEntryKind::File)
        .filter(|entry| {
            std::path::Path::new(entry.path.rel_path.as_str())
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
        })
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
            }
        })
        .collect())
}

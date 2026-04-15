use g3rs_code_ingestion_types::{G3RsCodeSourceChecksInput, G3RsSourceFile};

/// Build one `code` source checks input from selected metadata and source text.
pub(crate) fn assemble(
    rel_path: String,
    content: String,
    is_test: bool,
    profile_name: Option<String>,
    is_library_root: bool,
    is_shared_crate: bool,
) -> G3RsCodeSourceChecksInput {
    G3RsCodeSourceChecksInput {
        source_file: G3RsSourceFile {
            rel_path,
            content,
            is_test,
            profile_name,
            is_library_root,
        },
        is_shared_crate,
    }
}

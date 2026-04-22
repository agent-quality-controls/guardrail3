use g3rs_code_types::{
    G3RsCodeParsedSourceState, G3RsCodeSourceChecksInput, G3RsCodeWaiver, G3RsSourceFile,
};

/// Build one `code` source checks input from selected metadata and source text.
pub(crate) fn assemble(
    rel_path: String,
    content: String,
    parsed_source: G3RsCodeParsedSourceState,
    is_test: bool,
    profile_name: Option<String>,
    is_library_root: bool,
    is_shared_crate: bool,
    waivers: Vec<G3RsCodeWaiver>,
) -> G3RsCodeSourceChecksInput {
    G3RsCodeSourceChecksInput {
        source_file: G3RsSourceFile {
            rel_path,
            content,
            is_test,
            profile_name,
            is_library_root,
        },
        parsed_source,
        is_shared_crate,
        waivers,
    }
}

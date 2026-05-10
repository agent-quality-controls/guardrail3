use g3rs_code_types::{
    G3RsCodeParsedSourceState, G3RsCodeSourceChecksInput, G3RsCodeWaiver, G3RsSourceFile,
};

/// Bundle of per-source-file metadata threaded into `assemble`.
pub(crate) struct AssembleInputs {
    /// Workspace-root-relative path of the source file.
    pub(crate) rel_path: String,
    /// Raw file content as read from disk.
    pub(crate) content: String,
    /// Parsed Rust syntax-tree state for the file.
    pub(crate) parsed_source: G3RsCodeParsedSourceState,
    /// Whether the file belongs to a `[[test]]`/integration-test target.
    pub(crate) is_test: bool,
    /// Active rust policy profile name (`library`, `service`, ...) when known.
    pub(crate) profile_name: Option<String>,
    /// Whether the file is a `[lib]`-target root (`src/lib.rs` or its override).
    pub(crate) is_library_root: bool,
    /// Whether the owning crate opts into `package.metadata.guardrail3.shared = true`.
    pub(crate) is_shared_crate: bool,
    /// Waivers declared in the nearest enclosing `guardrail3-rs.toml`.
    pub(crate) waivers: Vec<G3RsCodeWaiver>,
}

/// Build one `code` source checks input from selected metadata and source text.
pub(crate) fn assemble(inputs: AssembleInputs) -> G3RsCodeSourceChecksInput {
    G3RsCodeSourceChecksInput {
        source_file: G3RsSourceFile {
            rel_path: inputs.rel_path,
            content: inputs.content,
            is_test: inputs.is_test,
            profile_name: inputs.profile_name,
            is_library_root: inputs.is_library_root,
        },
        parsed_source: inputs.parsed_source,
        is_shared_crate: inputs.is_shared_crate,
        waivers: inputs.waivers,
    }
}

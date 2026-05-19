use g3rs_code_types::{G3RsCodeSourceChecksInput, G3RsSourceFile};

/// Bundle of per-source-file metadata threaded into `assemble`.
pub(crate) struct AssembleInputs {
    /// Workspace-root-relative path of the source file.
    pub(crate) rel_path: String,
    /// Raw file content as read from disk.
    pub(crate) content: String,
    /// Whether the file belongs to a `[[test]]`/integration-test target.
    pub(crate) is_test: bool,
    /// Active rust policy profile name (`library`, `service`, ...) when known.
    pub(crate) profile_name: Option<String>,
    /// Whether the file is a `[lib]`-target root (`src/lib.rs` or its override).
    pub(crate) is_library_root: bool,
    /// Whether the owning crate opts into `package.metadata.guardrail3.shared = true`.
    pub(crate) is_shared_crate: bool,
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
        is_shared_crate: inputs.is_shared_crate,
    }
}

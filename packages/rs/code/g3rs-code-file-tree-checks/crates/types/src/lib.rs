/// One owned Cargo root measured for `code` structural caps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeStructuralCapRoot {
    /// Repo-relative root directory. Empty means the pointed workspace root.
    pub root_rel_dir: String,
    /// Repo-relative Cargo.toml path for this root.
    pub cargo_rel_path: String,
    /// Maximum module depth seen under this root.
    pub max_module_depth: usize,
    /// Maximum sibling source-directory count in one directory under this root.
    pub max_sibling_dirs: usize,
    /// Maximum sibling `.rs` file count in one directory under this root.
    pub max_sibling_rs_files: usize,
}

/// Input contract for `g3rs-code-file-tree-checks`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeFileTreeChecksInput {
    /// One structural-cap fact per owned root.
    pub roots: Vec<G3RsCodeStructuralCapRoot>,
}

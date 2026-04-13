#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeExceptionCommentFact {
    pub rel_path: String,
    pub line: usize,
    pub line_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeUnsafeCodeLintFact {
    pub cargo_rel_path: String,
    pub lint_level: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeConfigChecksInput {
    pub exception_comments: Vec<G3RsCodeExceptionCommentFact>,
    pub unsafe_code_lints: Vec<G3RsCodeUnsafeCodeLintFact>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsSourceFile {
    pub rel_path: String,
    pub content: String,
    pub is_test: bool,
    pub profile_name: Option<String>,
    pub is_library_root: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeSourceChecksInput {
    pub source_file: G3RsSourceFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeStructuralCapRoot {
    pub root_rel_dir: String,
    pub cargo_rel_path: String,
    pub max_module_depth: usize,
    pub max_sibling_dirs: usize,
    pub max_sibling_rs_files: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeFileTreeChecksInput {
    pub roots: Vec<G3RsCodeStructuralCapRoot>,
}

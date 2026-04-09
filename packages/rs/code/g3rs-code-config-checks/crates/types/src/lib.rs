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

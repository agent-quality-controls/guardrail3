use super::facts::{
    CodeInputFailureFacts, ExceptionCommentFacts, RustCodeFileFacts, UnsafeCodeLintFacts,
};

pub struct RustCodeFileInput<'a> {
    pub rel_path: &'a str,
    pub content: &'a str,
    pub ast: &'a syn::File,
    pub is_test: bool,
    pub profile_name: Option<&'a str>,
}

impl<'a> RustCodeFileInput<'a> {
    pub fn new(facts: &'a RustCodeFileFacts, content: &'a str, ast: &'a syn::File) -> Self {
        Self {
            rel_path: &facts.rel_path,
            content,
            ast,
            is_test: facts.is_test,
            profile_name: facts.profile_name.as_deref(),
        }
    }
}

pub struct UnsafeCodeLintInput<'a> {
    pub cargo_rel_path: &'a str,
    pub lint_level: Option<&'a str>,
}

impl<'a> UnsafeCodeLintInput<'a> {
    pub fn new(facts: &'a UnsafeCodeLintFacts) -> Self {
        Self {
            cargo_rel_path: &facts.cargo_rel_path,
            lint_level: facts.lint_level.as_deref(),
        }
    }
}

pub struct ExceptionCommentInput<'a> {
    pub rel_path: &'a str,
    pub line: usize,
    pub line_text: &'a str,
}

impl<'a> ExceptionCommentInput<'a> {
    pub fn new(facts: &'a ExceptionCommentFacts) -> Self {
        Self {
            rel_path: &facts.rel_path,
            line: facts.line,
            line_text: &facts.line_text,
        }
    }
}

pub struct CodeInputFailureInput<'a> {
    pub rel_path: &'a str,
    pub message: &'a str,
}

impl<'a> CodeInputFailureInput<'a> {
    pub fn new(facts: &'a CodeInputFailureFacts) -> Self {
        Self {
            rel_path: &facts.rel_path,
            message: &facts.message,
        }
    }
}

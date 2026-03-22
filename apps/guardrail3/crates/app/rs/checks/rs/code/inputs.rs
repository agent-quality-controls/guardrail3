use super::facts::{RustCodeFileFacts, UnsafeCodeLintFacts};

pub struct RustCodeFileInput<'a> {
    pub rel_path: &'a str,
    pub content: &'a str,
    pub ast: &'a syn::File,
    pub is_test: bool,
}

impl<'a> RustCodeFileInput<'a> {
    pub fn new(facts: &'a RustCodeFileFacts, content: &'a str, ast: &'a syn::File) -> Self {
        Self {
            rel_path: &facts.rel_path,
            content,
            ast,
            is_test: facts.is_test,
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

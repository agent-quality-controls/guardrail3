use super::facts::{
    CodeInputFailureFacts, ExceptionCommentFacts, RustCodeFileFacts, StructuralCapFacts,
    UnsafeCodeLintFacts,
};

pub struct RustCodeFileInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) content: &'a str,
    pub(crate) ast: &'a syn::File,
    pub(crate) is_test_root: bool,
    pub(crate) profile_name: Option<&'a str>,
}

impl<'a> RustCodeFileInput<'a> {
    pub fn new(facts: &'a RustCodeFileFacts, content: &'a str, ast: &'a syn::File) -> Self {
        Self {
            rel_path: &facts.rel_path,
            content,
            ast,
            is_test_root: facts.is_test_root,
            profile_name: facts.profile_name.as_deref(),
        }
    }
}

pub struct UnsafeCodeLintInput<'a> {
    pub(crate) cargo_rel_path: &'a str,
    pub(crate) lint_level: Option<&'a str>,
}

impl<'a> UnsafeCodeLintInput<'a> {
    pub fn new(facts: &'a UnsafeCodeLintFacts) -> Self {
        Self {
            cargo_rel_path: &facts.cargo_rel_path,
            lint_level: facts.lint_level.as_deref(),
        }
    }
}

pub struct StructuralCapInput<'a> {
    pub(crate) root_rel_dir: &'a str,
    pub(crate) cargo_rel_path: &'a str,
    pub(crate) max_module_depth: usize,
    pub(crate) max_sibling_dirs: usize,
    pub(crate) max_sibling_rs_files: usize,
}

impl<'a> StructuralCapInput<'a> {
    pub fn new(facts: &'a StructuralCapFacts) -> Self {
        Self {
            root_rel_dir: &facts.root_rel_dir,
            cargo_rel_path: &facts.cargo_rel_path,
            max_module_depth: facts.max_module_depth,
            max_sibling_dirs: facts.max_sibling_dirs,
            max_sibling_rs_files: facts.max_sibling_rs_files,
        }
    }
}

pub struct ExceptionCommentInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) line: usize,
    pub(crate) line_text: &'a str,
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
    pub(crate) rel_path: &'a str,
    pub(crate) message: &'a str,
}

impl<'a> CodeInputFailureInput<'a> {
    pub fn new(facts: &'a CodeInputFailureFacts) -> Self {
        Self {
            rel_path: &facts.rel_path,
            message: &facts.message,
        }
    }
}

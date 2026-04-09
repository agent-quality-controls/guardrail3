use g3rs_code_ast_checks_types::{G3RsCodeAstChecksInput, G3RsSourceFile};

pub(crate) struct G3RsCodeSourceFileAst {
    pub(crate) source_file: G3RsSourceFile,
    pub(crate) ast: syn::File,
}

pub(crate) struct CodeInputFailureRuleInput {
    pub(crate) rel_path: String,
    pub(crate) message: String,
}

#[derive(Clone, Copy)]
pub(crate) struct CodeSourceRuleInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) content: &'a str,
    pub(crate) ast: &'a syn::File,
    pub(crate) is_test: bool,
    #[allow(dead_code)] // reason: retained for upcoming profile-sensitive code AST rules
    pub(crate) profile_name: Option<&'a str>,
    #[allow(dead_code)] // reason: retained for upcoming lib.rs-only code AST rules
    pub(crate) is_library_root: bool,
}

impl<'a> From<&'a G3RsCodeSourceFileAst> for CodeSourceRuleInput<'a> {
    fn from(value: &'a G3RsCodeSourceFileAst) -> Self {
        Self {
            rel_path: &value.source_file.rel_path,
            content: &value.source_file.content,
            ast: &value.ast,
            is_test: value.source_file.is_test,
            profile_name: value.source_file.profile_name.as_deref(),
            is_library_root: value.source_file.is_library_root,
        }
    }
}

pub(crate) fn parse_input(
    input: &G3RsCodeAstChecksInput,
) -> Result<G3RsCodeSourceFileAst, syn::Error> {
    let ast = crate::parse::parse_rust_file(&input.source_file.content)?;
    Ok(G3RsCodeSourceFileAst {
        source_file: input.source_file.clone(),
        ast,
    })
}

pub(crate) fn parse_failure_input(
    input: &G3RsCodeAstChecksInput,
    parse_error: &syn::Error,
) -> CodeInputFailureRuleInput {
    CodeInputFailureRuleInput {
        rel_path: input.source_file.rel_path.clone(),
        message: format!("Failed to parse Rust source file: {parse_error}"),
    }
}

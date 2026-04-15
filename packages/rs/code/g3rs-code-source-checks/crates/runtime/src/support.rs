use g3rs_code_source_checks_types::{G3RsCodeSourceChecksInput, G3RsSourceFile};

pub(crate) struct G3RsCodeSourceFileAst {
    pub(crate) source_file: G3RsSourceFile,
    pub(crate) source: syn::File,
}

pub(crate) struct CodeInputFailureRuleInput {
    pub(crate) rel_path: String,
    pub(crate) message: String,
}

#[derive(Clone, Copy)]
pub(crate) struct CodeSourceRuleInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) content: &'a str,
    pub(crate) source: &'a syn::File,
    pub(crate) is_test: bool,
    pub(crate) is_shared_crate: bool,
    #[allow(dead_code)] // reason: retained for upcoming profile-sensitive code source rules
    pub(crate) profile_name: Option<&'a str>,
    #[allow(dead_code)] // reason: retained for upcoming lib.rs-only code source rules
    pub(crate) is_library_root: bool,
}

impl<'a> From<&'a G3RsCodeSourceFileAst> for CodeSourceRuleInput<'a> {
    fn from(value: &'a G3RsCodeSourceFileAst) -> Self {
        Self {
            rel_path: &value.source_file.rel_path,
            content: &value.source_file.content,
            source: &value.source,
            is_test: value.source_file.is_test,
            is_shared_crate: false,
            profile_name: value.source_file.profile_name.as_deref(),
            is_library_root: value.source_file.is_library_root,
        }
    }
}

pub(crate) fn parse_input(
    input: &G3RsCodeSourceChecksInput,
) -> Result<G3RsCodeSourceFileAst, syn::Error> {
    let source = crate::parse::parse_rust_file(&input.source_file.content)?;
    Ok(G3RsCodeSourceFileAst {
        source_file: input.source_file.clone(),
        source,
    })
}

pub(crate) fn parse_failure_input(
    input: &G3RsCodeSourceChecksInput,
    parse_error: &syn::Error,
) -> CodeInputFailureRuleInput {
    CodeInputFailureRuleInput {
        rel_path: input.source_file.rel_path.clone(),
        message: format!("Failed to parse Rust source file: {parse_error}"),
    }
}

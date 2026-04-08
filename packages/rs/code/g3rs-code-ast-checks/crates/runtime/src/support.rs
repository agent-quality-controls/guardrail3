use g3rs_code_ast_checks_types::{G3RsCodeAstChecksInput, G3RsSourceFile};

pub(crate) struct G3RsCodeSourceFileAst {
    pub(crate) source_file: G3RsSourceFile,
    pub(crate) ast: syn::File,
}

#[derive(Clone, Copy)]
pub(crate) struct CodeSourceRuleInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) content: &'a str,
    pub(crate) ast: &'a syn::File,
    pub(crate) is_test: bool,
}

impl<'a> From<&'a G3RsCodeSourceFileAst> for CodeSourceRuleInput<'a> {
    fn from(value: &'a G3RsCodeSourceFileAst) -> Self {
        Self {
            rel_path: &value.source_file.rel_path,
            content: &value.source_file.content,
            ast: &value.ast,
            is_test: value.source_file.is_test,
        }
    }
}

pub(crate) fn parse_input(input: &G3RsCodeAstChecksInput) -> Result<G3RsCodeSourceFileAst, syn::Error> {
    let ast = crate::parse::parse_rust_file(&input.source_file.content)?;
    Ok(G3RsCodeSourceFileAst {
        source_file: input.source_file.clone(),
        ast,
    })
}

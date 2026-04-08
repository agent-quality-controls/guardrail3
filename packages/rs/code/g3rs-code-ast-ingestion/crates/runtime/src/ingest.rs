use g3rs_code_ast_checks_types::{G3RsCodeAstChecksInput, G3RsSourceFile};

/// Build one `code` AST checks input from selected metadata and source text.
pub(crate) fn assemble(
    rel_path: String,
    content: String,
    is_test: bool,
    profile_name: Option<String>,
) -> G3RsCodeAstChecksInput {
    G3RsCodeAstChecksInput {
        source_file: G3RsSourceFile {
            rel_path,
            content,
            is_test,
            profile_name,
        },
    }
}

use g3rs_code_ingestion_types::{
    G3RsCodeConfigChecksInput, G3RsCodeExceptionCommentFact, G3RsCodeUnsafeCodeLintFact,
};

pub(crate) fn assemble(
    exception_comments: Vec<G3RsCodeExceptionCommentFact>,
    unsafe_code_lints: Vec<G3RsCodeUnsafeCodeLintFact>,
) -> G3RsCodeConfigChecksInput {
    G3RsCodeConfigChecksInput {
        exception_comments,
        unsafe_code_lints,
    }
}

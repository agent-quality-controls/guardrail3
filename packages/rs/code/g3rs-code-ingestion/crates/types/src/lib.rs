mod error;

pub use g3rs_code_source_checks_types::{G3RsCodeSourceChecksInput, G3RsSourceFile};
pub use g3rs_code_config_checks_types::{
    G3RsCodeConfigChecksInput, G3RsCodeExceptionCommentFact, G3RsCodeUnsafeCodeLintFact,
};

/// Placeholder file-tree-lane input until `g3rs-code-file-tree-checks` exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeFileTreeChecksInput;

#[cfg(feature = "api")]
pub use error::G3RsCodeIngestionError;

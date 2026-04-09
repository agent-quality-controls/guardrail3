mod error;

pub use g3rs_code_ast_checks_types::{G3RsCodeAstChecksInput, G3RsSourceFile};

/// Placeholder config-lane input until `g3rs-code-config-checks` exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeConfigChecksInput;

/// Placeholder file-tree-lane input until `g3rs-code-file-tree-checks` exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeFileTreeChecksInput;

#[cfg(feature = "api")]
pub use error::G3RsCodeIngestionError;

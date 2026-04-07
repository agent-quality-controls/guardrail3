/// Error types for parse failures.
mod error;
/// Filesystem boundary for file reading.
mod fs;
/// Parser entrypoints.
mod parser;

#[cfg(feature = "api")]
pub use clippy_toml_parser_types::{
    ArithmeticSideEffectsBinaryEntry, AwaitHoldingInvalidType, ClippyToml, DisallowedField,
    DisallowedFieldDetail, DisallowedPath, DisallowedPathDetail, InherentImplLintScope,
    MacroBraceEntry, MatchLintBehaviour, PubUnderscoreFieldsBehaviour, RenameEntry,
    SourceItemOrdering, SourceItemOrderingCategory, SourceItemOrderingModuleItemGroupings,
    SourceItemOrderingModuleItemKind, SourceItemOrderingTraitAssocItemKind,
    SourceItemOrderingTraitAssocItemKinds, SourceItemOrderingWithinModuleItemGroupings,
};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, parse};

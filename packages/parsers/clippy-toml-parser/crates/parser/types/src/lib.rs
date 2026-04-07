/// Typed clippy.toml model definitions.
mod clippy_toml;
use toml as _;

pub use clippy_toml::{
    ArithmeticSideEffectsBinaryEntry, AwaitHoldingInvalidType, ClippyToml, DisallowedField,
    DisallowedFieldDetail, DisallowedPath, DisallowedPathDetail, InherentImplLintScope,
    MacroBraceEntry, MatchLintBehaviour, PubUnderscoreFieldsBehaviour, RenameEntry,
    SourceItemOrdering, SourceItemOrderingCategory, SourceItemOrderingModuleItemGroupings,
    SourceItemOrderingModuleItemKind, SourceItemOrderingTraitAssocItemKind,
    SourceItemOrderingTraitAssocItemKinds, SourceItemOrderingWithinModuleItemGroupings,
};

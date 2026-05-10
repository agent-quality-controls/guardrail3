//! Shared types for the g3rs hooks family.

/// Hook-family input types.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3RsHookScriptKind, G3RsHooksConfigChecksInput, G3RsHooksFileTreeChecksInput,
    G3RsHooksScriptFileFact, G3RsHooksSelectedHookConfigFact, G3RsHooksSourceChecksInput,
};

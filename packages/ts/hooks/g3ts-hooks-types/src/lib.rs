/// Public type definitions for the G3TS hooks family.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3TsHookScriptKind, G3TsHooksConfigChecksInput, G3TsHooksEnabledCategories,
    G3TsHooksFileTreeChecksInput, G3TsHooksScriptFileFact, G3TsHooksSelectedHookConfigFact,
    G3TsHooksSourceChecksInput,
};

//! Shared types for the g3ts topology family.

/// Shared input/output type definitions consumed by topology runtimes.
#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3TsTopologyDescendantGuardrail3TsToml, G3TsTopologyFileTreeChecksInput,
    G3TsTopologyFileTreeInputFailure, G3TsTopologyNestedGuardrail3TsTomlInput,
};

//! Public re-export shim for the file-tree-checks input types.

#[cfg(feature = "api")]
pub use g3ts_topology_types::{
    G3TsTopologyDescendantGuardrail3TsToml, G3TsTopologyFileTreeChecksInput,
    G3TsTopologyFileTreeInputFailure, G3TsTopologyNestedGuardrail3TsTomlInput,
};

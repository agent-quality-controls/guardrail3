mod fs;
mod run;
mod view;

#[cfg(feature = "ingest")]
pub use run::ingest_for_file_tree_checks;
#[cfg(feature = "ingest")]
pub use g3rs_topology_types::{
    G3RsTopologyCargoManifestKind, G3RsTopologyFileTreeChecksInput,
    G3RsTopologyWorkspaceFamily, G3RsTopologyWorkspaceFamilyFileAttachment,
    G3RsTopologyWorkspaceFamilyFileKind,
};

#[cfg(test)]
use g3rs_topology_ingestion_assertions as _;

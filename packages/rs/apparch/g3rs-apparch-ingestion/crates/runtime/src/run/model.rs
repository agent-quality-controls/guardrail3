use cargo_toml_parser::CargoToml;
use g3rs_apparch_types::{
    G3RsApparchCrate, G3RsApparchDependencyEdge, G3RsApparchExternalDependency,
    G3RsApparchRustPolicyState,
};

#[derive(Debug, Clone)]
pub(super) struct WorkspaceRoot {
    pub(super) cargo: CargoToml,
    pub(super) rust_policy: G3RsApparchRustPolicyState,
}

#[derive(Debug, Clone)]
pub(super) struct CrateRecord {
    pub(super) krate: G3RsApparchCrate,
    pub(super) cargo: CargoToml,
}

#[derive(Debug, Default)]
pub(super) struct DependencyCollections {
    pub(super) internal_edges: Vec<G3RsApparchDependencyEdge>,
    pub(super) external_dependencies: Vec<G3RsApparchExternalDependency>,
}

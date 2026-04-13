#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsApparchLayer {
    Types,
    Logic,
    IoInbound,
    IoOutbound,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchCrate {
    pub crate_name: String,
    pub cargo_rel_path: String,
    pub rel_dir: String,
    pub layer: Option<G3RsApparchLayer>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchDependencyEdge {
    pub from_cargo_rel_path: String,
    pub to_cargo_rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchPublicTrait {
    pub cargo_rel_path: String,
    pub rel_path: String,
    pub trait_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchConfigChecksInput {
    pub crates: Vec<G3RsApparchCrate>,
    pub dependency_edges: Vec<G3RsApparchDependencyEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsApparchSourceChecksInput {
    pub crates: Vec<G3RsApparchCrate>,
    pub public_traits: Vec<G3RsApparchPublicTrait>,
}

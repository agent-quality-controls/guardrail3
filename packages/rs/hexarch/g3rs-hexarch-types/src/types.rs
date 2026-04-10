#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsHexarchLayer {
    Domain,
    Ports,
    App,
    Adapters,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsHexarchSourceCrateFacts {
    pub crate_name: String,
    pub rel_dir: String,
    pub layer: Option<G3RsHexarchLayer>,
    pub pub_trait_count: usize,
    pub public_free_fn_count: usize,
    pub public_inherent_method_count: usize,
    pub source_error_rel_path: Option<String>,
    pub source_error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsHexarchSourceChecksInput {
    pub crate_facts: G3RsHexarchSourceCrateFacts,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsHexarchConfigChecksInput;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsHexarchFileTreeChecksInput;

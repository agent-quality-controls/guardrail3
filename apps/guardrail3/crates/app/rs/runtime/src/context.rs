use guardrail3_app_rs_family_mapper::FamilyMapper;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_outbound_traits::ToolChecker;

#[cfg(feature = "family-hooks-shared")]
use std::path::Path;
#[cfg(feature = "family-hooks-shared")]
use guardrail3_outbound_traits::FileSystem;

pub(crate) struct RustRunContext<'a> {
    #[cfg(feature = "family-hooks-shared")]
    pub(crate) fs: &'a dyn FileSystem,
    #[cfg(feature = "family-hooks-shared")]
    pub(crate) path: &'a Path,
    pub(crate) tree: &'a ProjectTree,
    pub(crate) mapper: &'a FamilyMapper<'a>,
    pub(crate) tc: &'a dyn ToolChecker,
    pub(crate) thorough: bool,
}

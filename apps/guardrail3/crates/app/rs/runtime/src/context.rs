#[cfg(feature = "routing")]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(feature = "routing")]
use guardrail3_app_rs_legality::RustLegalityFacts;
#[cfg(feature = "family-hooks-shared")]
use guardrail3_outbound_traits::FileSystem;
#[cfg(any(
    feature = "family-deps",
    feature = "family-test",
    feature = "family-release",
    feature = "family-hooks-shared",
    feature = "family-hooks-rs",
))]
use guardrail3_outbound_traits::ToolChecker;
#[cfg(feature = "family-hooks-shared")]
use std::path::Path;

pub(crate) struct RustRunContext<'a> {
    #[cfg(feature = "family-hooks-shared")]
    pub(crate) fs: &'a dyn FileSystem,
    #[cfg(feature = "family-hooks-shared")]
    pub(crate) path: &'a Path,
    #[cfg(feature = "routing")]
    pub(crate) legality: &'a RustLegalityFacts,
    #[cfg(feature = "routing")]
    pub(crate) mapper: &'a FamilyMapper<'a>,
    #[cfg(any(
        feature = "family-deps",
        feature = "family-test",
        feature = "family-release",
        feature = "family-hooks-shared",
        feature = "family-hooks-rs",
    ))]
    pub(crate) tc: &'a dyn ToolChecker,
    #[cfg(feature = "family-release")]
    pub(crate) thorough: bool,
}

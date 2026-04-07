/// Error surface for parser failures.
mod error;
/// Centralized filesystem boundary for parser file reads.
mod fs;
/// Parser module facade.
mod parser;

#[cfg(feature = "api")]
pub use deny_toml_parser_types::{
    AdvisoriesConfig, AdvisoryIgnoreDetail, AdvisoryIgnoreEntry, AdvisoryScope, BanAllowDetail,
    BanAllowEntry, BanBuildAllowBuildScriptDetail, BanBuildAllowBuildScriptEntry,
    BanBuildBypassAllowEntry, BanBuildBypassEntry, BanBuildConfig, BanDenyDetail, BanDenyEntry,
    BanFeatureEntry, BanSkipDetail, BanSkipEntry, BanSkipTreeDetail, BanSkipTreeEntry,
    BanWorkspaceDependenciesConfig, BansConfig, DenyToml, GitSpec, GraphConfig,
    GraphTargetDetail, GraphTargetEntry, LicenseClarification, LicenseClarificationFile,
    LicenseException, LicensesConfig, LicensesPrivateConfig, OutputConfig, SourcesAllowOrg,
    SourcesConfig,
};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, parse};
#[cfg(feature = "api")]
pub use toml::Value;

/// Error surface for parser failures.
mod error;
/// Centralized filesystem boundary for parser file reads.
mod fs;
/// Parser module facade.
mod parser;

#[cfg(feature = "api")]
pub use deny_toml_parser_types::{
    AdvisoriesConfig, AdvisoryIgnoreDetail, AdvisoryIgnoreEntry, BanAllowDetail, BanAllowEntry,
    BanDenyDetail, BanDenyEntry, BanFeatureEntry, BanSkipDetail, BanSkipEntry, BansConfig,
    DenyToml, GraphConfig, LicenseException, LicensesConfig, LicensesPrivateConfig, OutputConfig,
    SourcesConfig,
};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, parse};
#[cfg(feature = "api")]
pub use toml::Value;

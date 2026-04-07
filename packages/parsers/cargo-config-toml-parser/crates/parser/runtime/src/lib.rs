/// Error surface for parser failures.
mod error;
/// Centralized filesystem boundary for parser file reads.
mod fs;
/// Parser module facade.
mod parser;

#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, parse};
#[cfg(feature = "api")]
pub use cargo_config_toml_parser_types::{
    BuildConfig, CacheConfig, CargoConfigToml, CargoNewConfig, CommandValue, EnvValue,
    EnvValueDetail, FutureIncompatReportConfig, HttpConfig, HttpSslVersion, HttpTlsRange,
    IncludeEntry, IncludePath, InstallConfig, IntegerOrBool, IntegerOrString, NetConfig,
    NetSshConfig, ProfileConfig, ProfileSettings, RegistryConfig, RegistryDefaults, ResolverConfig,
    SourceConfig, StringOrBool, TargetConfig, TargetSelector, TermConfig, TermProgressConfig,
};
#[cfg(feature = "api")]
pub use toml::Value;

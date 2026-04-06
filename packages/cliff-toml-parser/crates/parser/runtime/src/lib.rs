/// Error types for parse failures.
mod error;
/// Filesystem boundary for file reading.
mod fs;
/// Parser entrypoints.
mod parser;

#[cfg(feature = "api")]
pub use cliff_toml_parser_types::{
    CliffChangelogSection, CliffCommitParser, CliffGitSection, CliffToml,
};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, parse};
#[cfg(feature = "api")]
pub use toml::Value;

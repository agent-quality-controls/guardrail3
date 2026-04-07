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
pub use rustfmt_toml_parser_types::{
    BraceStyle, Color, ControlBraceStyle, Edition, EmitMode, FloatLiteralTrailingZero,
    GroupImportsTactic, Heuristics, HexLiteralCase, ImportGranularity, IndentStyle,
    MatchArmLeadingPipe, NewlineStyle, RustfmtToml, StyleEdition, Version,
};
#[cfg(feature = "api")]
pub use toml::Value;

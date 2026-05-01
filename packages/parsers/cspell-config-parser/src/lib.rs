#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use cspell_config_parser_runtime::{
    Error, from_path_document, parse_document, parse_error_reason, typed,
};

#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use npmrc_parser_runtime::{
    Error, effective_value, from_path, from_path_document, parse, parse_document,
    parse_error_reason, typed,
};

#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use jscpd_json_parser_runtime::{
    Error, from_path, from_path_document, parse, parse_document, parse_error_reason, typed,
};

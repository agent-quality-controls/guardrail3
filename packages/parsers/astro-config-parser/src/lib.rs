#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use astro_config_parser_runtime::{
    Error, from_path, has_integration, integration, parse, parse_document, parse_error_reason,
    typed,
};

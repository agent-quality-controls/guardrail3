#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use clippy_toml_parser_runtime::{
    Error, ban_section, bool_setting, from_path, parse, parse_document, parse_error_reason,
    top_level_keys, typed,
};

#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use tsconfig_json_parser_runtime::{
    Error, bool_field_state, compiler_options, extends_entries, from_path, from_path_document,
    parse, parse_document, parse_error_reason, typed,
};

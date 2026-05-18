#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use syncpack_config_parser_runtime::{
    Error, first_matching_group_pins_dependency, from_path, from_path_document, parse,
    parse_document, parse_error_reason, pattern_list_matches, typed,
};

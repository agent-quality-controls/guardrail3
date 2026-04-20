#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use eslint_config_parser_runtime::{
    Error, from_path, parse, parse_document, parse_error_reason, probe, rule_setting, typed,
};

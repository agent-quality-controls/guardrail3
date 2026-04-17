#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use cargo_toml_parser_runtime::{
    document, Error, from_path, parse, parse_document,
};

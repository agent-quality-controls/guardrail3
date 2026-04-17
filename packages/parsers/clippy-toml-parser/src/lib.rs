#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use clippy_toml_parser_runtime::{Error, from_path, parse};

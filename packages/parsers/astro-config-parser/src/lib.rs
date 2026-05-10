//! Facade crate for typed Astro config parsing.
#![allow(
    clippy::multiple_crate_versions,
    reason = "swc_common pins siphasher 0.3.11 while criterion (dev-tree transitive) pulls siphasher 1.0.2; resolving requires upstream swc bump"
)]

#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use astro_config_parser_runtime::{
    Error, from_path, has_integration, integration, module_has_runtime_source_import, parse,
    parse_document, parse_error_reason, typed,
};

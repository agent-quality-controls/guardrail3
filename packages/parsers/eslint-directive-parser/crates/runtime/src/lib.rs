#![allow(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::cast_possible_truncation,
    clippy::excessive_nesting,
    clippy::indexing_slicing,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::string_slice,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::unnecessary_wraps,
    reason = "parser runtime keeps a uniform fallible parser facade and byte-marker scanning"
)]

mod document;
mod error;
mod fs;
mod parser;
#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use document::{parse_error_reason, typed};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, from_path_document, parse, parse_document};

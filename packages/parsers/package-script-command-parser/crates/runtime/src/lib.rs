#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::arithmetic_side_effects,
    clippy::type_complexity,
    clippy::unnecessary_wraps,
    reason = "parser runtime keeps the same public parser facade as other parser packages"
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

#[cfg(test)]
use tempfile as _;

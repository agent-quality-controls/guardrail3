#![allow(
    clippy::excessive_nesting,
    clippy::missing_docs_in_private_items,
    reason = "this file mirrors nextest.toml schema directly; field names intentionally track the file shape"
)]

pub mod basics;
mod document;
pub mod execution;
pub mod profile;
pub mod scripts;

pub use document::NextestToml;

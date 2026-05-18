#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::shadow_unrelated,
    clippy::type_complexity,
    reason = "parser runtime keeps the same public parser facade as other parser packages"
)]
#![cfg_attr(
    test,
    allow(
        clippy::expect_used,
        clippy::needless_raw_string_hashes,
        reason = "parser runtime tests use panic-based assertions"
    )
)]

#[cfg(feature = "api")]
pub mod error;
#[cfg(feature = "api")]
pub mod fs;
#[cfg(feature = "api")]
pub mod matcher;
#[cfg(feature = "api")]
pub mod parser;
#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use matcher::{first_matching_group_pins_dependency, pattern_list_matches};
#[cfg(feature = "api")]
pub use parser::{from_path, from_path_document, parse, parse_document, parse_error_reason, typed};

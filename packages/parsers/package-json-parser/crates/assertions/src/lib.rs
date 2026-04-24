#![allow(
    clippy::assertions_on_constants,
    clippy::missing_assert_message,
    clippy::missing_docs_in_private_items,
    clippy::missing_panics_doc,
    clippy::needless_pass_by_value,
    clippy::too_many_arguments,
    clippy::wildcard_enum_match_arm,
    reason = "assertion helper crates are panic-based proof sites for parser tests"
)]

use package_json_parser_runtime as _;

#[cfg(feature = "checks")]
pub mod parser;

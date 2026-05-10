//! Assertion helpers for the Astro config parser.
#![allow(
    clippy::multiple_crate_versions,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_assert_message,
    clippy::module_name_repetitions,
    reason = "multiple_crate_versions arises from swc_common siphasher 0.3.11 vs criterion-transitive siphasher 1.0.2 (requires upstream swc bump). assertion helpers panic by name and the snapshot equality asserts are positional Vec/Option comparisons whose expected values are constructed in-test, so per-field messages would duplicate the assert_eq output. missing_errors_doc would duplicate the crate Error enum docs."
)]

use astro_config_parser_runtime as _;

#[cfg(feature = "checks")]
pub mod parser;

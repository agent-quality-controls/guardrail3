//! Assertion helpers for g3rs-apparch ingestion.
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    reason = "assertion helpers panic when the ingested apparch facts diverge from expected; the panic surface is documented by the assert_* function name and each module's docstring"
)]

#[cfg(feature = "ingest")]
use g3rs_apparch_ingestion_runtime as _;
use guardrail3_check_types as _;

#[cfg(feature = "ingest")]
pub mod run;
